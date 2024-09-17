//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-06-08
//!
//! Description: Windows platform implementation of keyevent

#![cfg(windows)]
#![allow(unused)]

use log::{debug, error, info, warn};
use windows::Win32::UI::Accessibility::IUIAutomationElement;

extern crate windows;
use crate::consts;
use keycode::{KeyMap, KeyMappingId};
use lazy_static::lazy_static;
use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use windows::Win32::Foundation::{LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::EM_GETSEL;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{
        VK_C, VK_CONTROL, VK_LCONTROL, VK_LMENU, VK_LSHIFT, VK_MENU,
        VK_RCONTROL, VK_RMENU, VK_RSHIFT, VK_SHIFT,
    },
    WindowsAndMessaging::{
        CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION,
        HHOOK, KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT, WH_KEYBOARD_LL, WH_MOUSE_LL,
        WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
        WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
};
use windows_core::{Interface, PWSTR};
// use windows::Win32::UI::Controls::RichEdit::{EM_EXGETSEL, CHARRANGE};
// use windows::Win32::UI::Input::KeyboardAndMouse::GetFocus;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED,
};
use windows::Win32::UI::Accessibility::{
    CUIAutomation8, IUIAutomation, IUIAutomationTextEditPattern,
    IUIAutomationTextPattern, IUIAutomationTextPattern2, UIA_TextEditPatternId,
    UIA_TextPattern2Id, UIA_TextPatternId,
};
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
use windows::Win32::UI::WindowsAndMessaging::SendMessageW;

struct Manager {
    kb_hook_id: Mutex<Option<HHOOK>>,
    callbacks:
        Arc<Mutex<HashMap<consts::Shortcut, Arc<dyn Fn() + Send + Sync>>>>,
    modifier_keys: Mutex<HashSet<u32>>,
    cpcp_last_time: Mutex<u128>,
    alt_last_time: Mutex<u128>,

    mouse_hook_id: Mutex<Option<HHOOK>>,
    mouse_callbacks: Mutex<
        HashMap<
            consts::MouseEvent,
            Box<dyn Fn(super::MouseEventInfo) + Send + Sync>,
        >,
    >,
}

lazy_static! {
    static ref MANAGER: Manager = Manager {
        kb_hook_id: Mutex::new(None),
        callbacks: Arc::new(Mutex::new(HashMap::new())),
        modifier_keys: Mutex::new(HashSet::new()),
        cpcp_last_time: Mutex::new(0),
        alt_last_time: Mutex::new(0),
        mouse_hook_id: Mutex::new(None),
        mouse_callbacks: Mutex::new(HashMap::new()),
    };
}
unsafe impl Sync for Manager {}

impl Manager {
    fn is_modifier_key(vkcode: &u32) -> bool {
        match *vkcode as u16 {
            vk if vk == VK_CONTROL.0
                || vk == VK_LCONTROL.0
                || vk == VK_RCONTROL.0
                || vk == VK_MENU.0
                || vk == VK_LMENU.0
                || vk == VK_RMENU.0 =>
            {
                true
            }
            _ => false,
        }
    }

    fn set_modifier_pressed(&self, vkcode: u32) -> bool {
        let mut modifier_keys = self.modifier_keys.lock().unwrap();
        if modifier_keys.contains(&vkcode) {
            return false;
        }
        modifier_keys.insert(vkcode);
        return true;
    }

    fn unset_modifier_pressed(&self, vkcode: &u32) {
        let mut modifier_keys = self.modifier_keys.lock().unwrap();
        modifier_keys.remove(vkcode);
    }
    fn try_trigger_cpcp(&self, vkcode: &u32) -> bool {
        let modifier_keys = self.modifier_keys.lock().unwrap();
        let controls = [VK_CONTROL.0, VK_LCONTROL.0, VK_RCONTROL.0];
        if !controls
            .iter()
            .any(|&control| modifier_keys.contains(&(control as u32)))
        {
            return false;
        }
        if *vkcode != VK_C.0 as u32 {
            return false;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        {
            let mut last_time = self.cpcp_last_time.lock().unwrap();
            // println!("now: {}, last_time: {}", now, *last_time);
            if now - *last_time >= 350 {
                *last_time = now;
                return false;
            }
            *last_time = 0;
        }

        let cbs = self.callbacks.lock().unwrap();
        if let Some(callback) = cbs.get(&consts::Shortcut::DoubleCopy) {
            callback();
            return true;
        }
        false
    }

    fn try_trigger_double_alt(&self, vkcode: &u32) -> bool {
        println!("try_trigger_double_alt: {}", vkcode);
        let controls = [VK_RMENU.0, VK_LMENU.0, VK_MENU.0];
        if !controls.iter().any(|&control| *vkcode == control as u32) {
            return false;
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        {
            let mut last_time = self.alt_last_time.lock().unwrap();

            println!(
                "vkcode: {}, now: {}, last_time: {}",
                vkcode, now, *last_time
            );
            if now - *last_time >= consts::DOUBLE_CLICK_INTERVAL as u128 {
                *last_time = now;
                return false;
            }
            *last_time = 0;
        }

        let cbs = self.callbacks.lock().unwrap();
        if let Some(callback) = cbs.get(&consts::Shortcut::DoubleAlt) {
            let callback_clone = callback.clone();
            std::thread::spawn(move || {
                callback_clone();
            });
            return true;
        }
        false
    }

    fn register(
        &self,
        shortcut: consts::Shortcut,
        cb: Arc<dyn Fn() + Send + Sync>,
    ) {
        let mut cbs = self.callbacks.lock().unwrap();
        cbs.insert(shortcut, cb);

        if *self.kb_hook_id.lock().unwrap() == None {
            self.set_key_hooks();
        }
    }

    fn unregister(&self, shortcut: consts::Shortcut) {
        let mut cbs = self.callbacks.lock().unwrap();
        cbs.remove(&shortcut);

        if cbs.is_empty() {
            self.unset_key_hooks();
        }
    }

    fn set_key_hooks(&self) {
        unsafe {
            let module_handle = GetModuleHandleW(PWSTR::null());

            let hhook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(hook_proc),
                module_handle.unwrap(),
                0,
            );
            match hhook {
                Ok(hookid) => {
                    *self.kb_hook_id.lock().unwrap() = Some(hookid);
                }
                Err(e) => {
                    error!("Failed to set hook: {:?}", e);
                }
            }
        }
    }

    fn unset_key_hooks(&self) {
        unsafe {
            if let Some(hookid) = self.kb_hook_id.lock().unwrap().take() {
                UnhookWindowsHookEx(hookid).unwrap();
            }
        }
    }

    fn set_mouse_hooks(&self) {
        unsafe {
            let module_handle = GetModuleHandleW(PWSTR::null());

            let hhook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                module_handle.unwrap(),
                0,
            );
            println!("set_mouse_hooks: {:?}", hhook);
            match hhook {
                Ok(hookid) => {
                    *self.mouse_hook_id.lock().unwrap() = Some(hookid);
                }
                Err(e) => {
                    error!("Failed to set mouse hook: {:?}", e);
                }
            }
        }
    }

    fn unset_mouse_hooks(&self) {
        unsafe {
            if let Some(hookid) = self.mouse_hook_id.lock().unwrap().take() {
                UnhookWindowsHookEx(hookid).unwrap();
            }
        }
    }

    fn register_mouse_event(
        &self,
        event: consts::MouseEvent,
        cb: Box<dyn Fn(super::MouseEventInfo) + Send + Sync>,
    ) {
        let mut cbs = self.mouse_callbacks.lock().unwrap();
        cbs.insert(event, cb);

        if *self.mouse_hook_id.lock().unwrap() == None {
            self.set_mouse_hooks();
        }
    }

    fn unregister_mouse_event(&self, event: consts::MouseEvent) {
        let mut cbs = self.mouse_callbacks.lock().unwrap();
        cbs.remove(&event);

        if cbs.is_empty() {
            self.unset_mouse_hooks();
        }
    }

    fn on_mouse_event(&self, event_type: u32, event_info: &MSLLHOOKSTRUCT) {
        match event_type {
            WM_MOUSEMOVE => {
                let cbs = self.mouse_callbacks.lock().unwrap();
                if let Some(callback) = cbs.get(&consts::MouseEvent::Move) {
                    callback(super::MouseEventInfo {
                        pt: super::Point {
                            x: event_info.pt.x,
                            y: event_info.pt.y,
                        },
                    });
                }
            }
            WM_LBUTTONDOWN => {
                let cbs = self.mouse_callbacks.lock().unwrap();
                if let Some(callback) = cbs.get(&consts::MouseEvent::LeftDown) {
                    callback(super::MouseEventInfo {
                        pt: super::Point {
                            x: event_info.pt.x,
                            y: event_info.pt.y,
                        },
                    });
                }
            }
            WM_LBUTTONUP => {
                let cbs = self.mouse_callbacks.lock().unwrap();
                if let Some(callback) = cbs.get(&consts::MouseEvent::LeftUp) {
                    callback(super::MouseEventInfo {
                        pt: super::Point {
                            x: event_info.pt.x,
                            y: event_info.pt.y,
                        },
                    });
                }
            }
            _ => {}
        }
    }

    fn clear(&self) {
        self.callbacks.lock().unwrap().clear();
        self.unset_key_hooks();

        self.mouse_callbacks.lock().unwrap().clear();
        self.unset_mouse_hooks();
    }
}

fn on_kb_event(wparam: WPARAM, lparam: LPARAM) {
    // debug!("on_kb_event: wparam: {}, lparam: {}", wparam.0, lparam.0);

    let kb: *const KBDLLHOOKSTRUCT =
        lparam.0 as *const usize as *const KBDLLHOOKSTRUCT;
    // unsafe { std::mem::transmute(lparam.0 as usize) };

    let vk_code = unsafe { (*kb).vkCode };

    match wparam.0 as u32 {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            // debug!("key down: {}", vk_code);
            if Manager::is_modifier_key(&vk_code) {
                debug!("modifier key down: {}", vk_code);
                if MANAGER.set_modifier_pressed(vk_code.clone()) {
                    MANAGER.try_trigger_double_alt(&vk_code);
                }
            } else {
                // debug!("not modifier key down: {}", vk_code);
                MANAGER.try_trigger_cpcp(&vk_code);
            }
        }
        WM_KEYUP | WM_SYSKEYUP => {
            debug!("key up: {}", vk_code);
            if Manager::is_modifier_key(&vk_code) {
                MANAGER.unset_modifier_pressed(&vk_code);
                println!(
                    "modifier key up: {}, {:?}",
                    vk_code,
                    MANAGER.modifier_keys.lock().unwrap()
                );
            }
        }
        _ => {}
    }
}

unsafe extern "system" fn hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    debug!("hook_proc: {}, {:?}, {:?}", code, wparam, lparam);

    if code == HC_ACTION as i32 {
        on_kb_event(wparam.clone(), lparam.clone());
    }

    // debug!("CallNextHookEx ..., {:?}|{:?}", wparam, lparam);
    CallNextHookEx(None, code, wparam, lparam)
}

unsafe extern "system" fn mouse_hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if (code == HC_ACTION as i32) {
        let ms: *const MSLLHOOKSTRUCT =
            lparam.0 as *const usize as *const MSLLHOOKSTRUCT;
        MANAGER.on_mouse_event(wparam.0 as u32, &*ms);
    }
    CallNextHookEx(None, code, wparam, lparam)
}

pub fn register_copy_copy<T>(cb: T)
where
    T: Fn() + Send + Sync + 'static,
{
    MANAGER.register(consts::Shortcut::DoubleCopy, Arc::new(cb));
}

pub fn unregister_copy_copy() {
    MANAGER.unregister(consts::Shortcut::DoubleCopy);
}

pub fn register_double_alt<T: Fn() + Send + Sync + 'static>(cb: T) {
    MANAGER.register(consts::Shortcut::DoubleAlt, Arc::new(cb));
}

pub fn unregister_double_alt() {
    MANAGER.unregister(consts::Shortcut::DoubleAlt);
}

pub fn register_mouse_event<
    T: Fn(super::MouseEventInfo) + Send + Sync + 'static,
>(
    event: consts::MouseEvent,
    cb: T,
) {
    MANAGER.register_mouse_event(event, Box::new(cb))
}

pub fn unregister_mouse_event(event: consts::MouseEvent) {
    MANAGER.unregister_mouse_event(event);
}

pub fn get_focus_edit_text() -> String {
    let hwnd = unsafe { GetForegroundWindow() };

    println!("hwnd: {:?}", hwnd.0);
    if hwnd.0 == std::ptr::null_mut() {
        return "".to_string();
    }

    let mut startpos: usize = 0;
    let mut endpos: usize = 0;

    unsafe {
        let wparam = WPARAM(&mut startpos as *mut _ as usize);
        let lparam = LPARAM(&mut endpos as *mut _ as isize);
        SendMessageW(hwnd, EM_GETSEL, wparam, lparam);
        println!("select range: {:?}, {:?}", startpos, endpos);
    }

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED);
        let uia = CoCreateInstance::<_, IUIAutomation>(
            &CUIAutomation8,
            None,
            CLSCTX_INPROC_SERVER,
        )
        .unwrap();
        let element = uia.ElementFromHandle(hwnd).unwrap();
        let focused_element = uia.GetFocusedElement().unwrap();

        let focused_w = focused_element.CurrentNativeWindowHandle().unwrap();
        println!("focused_w: {:?}, {:?}", focused_w.0, hwnd.0);

        get_text_pattern_text(&focused_element);
        // get_text_pattern2_text(&focused_element);
        // get_text_edit_pattern_text(&focused_element);

        CoUninitialize();
    }
    "".to_string()
}

unsafe fn get_text_pattern_text(
    focused_element: &IUIAutomationElement,
) -> String {
    match focused_element.GetCurrentPattern(UIA_TextPatternId) {
        Ok(text_pattern) => {
            let tp = text_pattern.cast::<IUIAutomationTextPattern>().unwrap();
            println!("focused_element has text pattern");
            match tp.GetSelection() {
                Ok(selection) => {
                    println!("selection: {:?}", selection.Length());
                    if selection.Length().unwrap() > 0 {
                        let range = selection.GetElement(0).unwrap();
                        let text = range.GetText(-1).unwrap();
                        println!("text: {:?}", text.to_string());
                        return text.to_string();
                    }
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("focused_element has no text pattern: {:?}", e);
        }
    }
    "".to_string()
}

unsafe fn get_text_pattern2_text(
    focused_element: &IUIAutomationElement,
) -> String {
    match focused_element.GetCurrentPattern(UIA_TextPattern2Id) {
        Ok(text_pattern) => {
            let tp = text_pattern.cast::<IUIAutomationTextPattern2>().unwrap();
            println!("focused_element has text pattern");
            match tp.GetSelection() {
                Ok(selection) => {
                    println!("se请提供需要翻译的文本。lection: {:?}", selection.Length());
                    if selection.Length().unwrap() > 0 {
                        let range = selection.GetElement(0).unwrap();
                        let text = range.GetText(-1).unwrap();
                        println!("text: {:?}", text.to_string());
                        return text.to_string();
                    }
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("focused_element has no text pattern: {:?}", e);
        }
    }
    "".to_string()
}

unsafe fn get_text_edit_pattern_text(
    focused_element: &IUIAutomationElement,
) -> String {
    match focused_element.GetCurrentPattern(UIA_TextEditPatternId) {
        Ok(text_pattern) => {
            let tp =
                text_pattern.cast::<IUIAutomationTextEditPattern>().unwrap();
            println!("focused_element has text pattern");
            match tp.GetSelection() {
                Ok(selection) => {
                    println!("selection: {:?}", selection.Length());
                    if selection.Length().unwrap() > 0 {
                        let range = selection.GetElement(0).unwrap();
                        let text = range.GetText(-1).unwrap();
                        println!("text: {:?}", text.to_string());
                        return text.to_string();
                    }
                }
                Err(e) => {
                    println!("error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("focused_element has no text pattern: {:?}", e);
        }
    }
    "".to_string()
}

pub fn clear() {
    MANAGER.clear();
}
