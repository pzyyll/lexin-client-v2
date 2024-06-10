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

extern crate windows;
use keycode::{KeyMap, KeyMappingId};
use lazy_static::lazy_static;
use std::cell::OnceCell;
use std::collections::HashSet;
use std::sync::Mutex;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{VK_C, VK_CONTROL, VK_LCONTROL, VK_RCONTROL},
    WindowsAndMessaging::{
        CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION,
        HHOOK, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
        WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
};
use windows_core::PWSTR;

struct Manager {
    kb_hook_id: Mutex<Option<HHOOK>>,
    cp_callback: Mutex<Option<Box<dyn Fn() + Send + Sync>>>,
    modifier_keys: Mutex<HashSet<u32>>,
    cpcp_last_time: Mutex<u128>,
}

lazy_static! {
    static ref MANAGER: Manager = Manager {
        kb_hook_id: Mutex::new(None),
        cp_callback: Mutex::new(None),
        modifier_keys: Mutex::new(HashSet::new()),
        cpcp_last_time: Mutex::new(0),
    };
}
unsafe impl Sync for Manager {}

impl Manager {
    fn is_modifier_key(vkcode: u32) -> bool {
        match vkcode as u16 {
            vk if vk == VK_CONTROL.0
                || vk == VK_LCONTROL.0
                || vk == VK_RCONTROL.0 =>
            {
                true
            }
            _ => false,
        }
    }

    fn set_modifier_pressed(&self, vkcode: u32) {
        let mut modifier_keys = self.modifier_keys.lock().unwrap();
        if modifier_keys.contains(&vkcode) {
            return;
        }
        modifier_keys.insert(vkcode);
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
        let mut last_time = self.cpcp_last_time.lock().unwrap();

        // println!("now: {}, last_time: {}", now, *last_time);
        if now - *last_time >= 350 {
            *last_time = now;
            return false;
        }
        *last_time = 0;
        match &*self.cp_callback.lock().unwrap() {
            Some(cb) => {
                cb();
                return true;
            }
            None => {
                return false;
            }
        }
    }
}

fn on_kb_event(wparam: WPARAM, lparam: LPARAM) {
    unsafe {
        let kb: *const KBDLLHOOKSTRUCT =
            std::mem::transmute::<LPARAM, *const KBDLLHOOKSTRUCT>(lparam);
        match wparam.0 as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => {
                // println!("key down: {}", (*kb).vkCode);
                if Manager::is_modifier_key((*kb).vkCode) {
                    MANAGER.set_modifier_pressed((*kb).vkCode);
                } else {
                    MANAGER.try_trigger_cpcp(&(*kb).vkCode);
                }
            }
            WM_KEYUP | WM_SYSKEYUP => {
                // println!("key up: {}", (*kb).vkCode);
                if Manager::is_modifier_key((*kb).vkCode) {
                    MANAGER.unset_modifier_pressed(&(*kb).vkCode);
                }
            }
            _ => {}
        }
    }
}

unsafe extern "system" fn hook_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        if code == HC_ACTION as i32 {
            on_kb_event(wparam, lparam);
        }
        CallNextHookEx(None, code, wparam, lparam)
    }
}

fn set_hooks() {
    unsafe {
        let module_handle = GetModuleHandleW(PWSTR::null());

        let hhook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(hook_proc),
            module_handle.unwrap(),
            0,
        )
        .unwrap();

        *MANAGER.kb_hook_id.lock().unwrap() = Some(hhook);
    }
}

fn unset_hooks() {
    unsafe {
        if let Some(hookid) = *MANAGER.kb_hook_id.lock().unwrap() {
            UnhookWindowsHookEx(hookid).unwrap();
        }
    }
}

pub fn register_copy_copy<T>(cb: T)
where
    T: Fn() + Send + Sync + 'static,
{
    unsafe {
        if MANAGER.kb_hook_id.lock().unwrap().is_none() {
            set_hooks();
        }
        *MANAGER.cp_callback.lock().unwrap() = Some(Box::new(cb));
    }
}

pub fn unregister_copy_copy() {
    unsafe {
        *MANAGER.cp_callback.lock().unwrap() = None;
        unset_hooks();
    }
}
