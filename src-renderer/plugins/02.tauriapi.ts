import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getCurrent } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";
import { Store } from "@tauri-apps/plugin-store";
import { listen } from "@tauri-apps/api/event";

export class Tauri {
  store: Store;
  constructor() {
    this.store = new Store("setting.bin");
  }

  async openWindow(url: string, label: string, title: string = "New Window") {
    console.log("Opening window", url, label, title);
    const webviewWindow = new WebviewWindow(label, {
      url: url,
      title: title,
    });

    // Creating a promise resolves on creation success and failure.
    const created = new Promise((resolve, reject) => {
      webviewWindow.once("tauri://created", () => {
        console.log("Window created");
        resolve(webviewWindow);
      });
      webviewWindow.once("tauri://error", (e) => {
        console.error("Error creating window", e);
        reject(e);
      });
    });

    return created;
  }

  async openPlayground() {
    console.log("Opening playground");
    return await this.openWindow("/playground", "playground", "Tauri Playground");
  }

  async openDevLab() {
    console.log("Opening dev lab");
    return await this.openWindow("/devlab", "devlab", "Tauri Dev Lab");
  }

  async openTranslateWindow() {
    console.log("Opening translate window");
    return await invoke("open_translate_window", {});
  }

  async resizeWindowHeight(height: number) {
    return await invoke("resize_window_height", { height });
  }

  async resizeWindowWidth(width: number) {
    return await invoke("resize_window_width", { width });
  }

  async getWindowSize() {
    const window = getCurrent();
    return (await window.innerSize()).toLogical(await window.scaleFactor());
  }

  async onTranslateInputResize({ width, height }: { width: number; height: number }) {
    const window = getCurrent();
    const cursize = (await window.innerSize()).toLogical(await window.scaleFactor());
    if (!(await window.isMaximized()) && cursize.height < height) {
      await this.resizeWindowHeight(height);
    }
  }

  async getAppSettings() {
    return await invoke("get_settings", {});
  }

  async getAppStore(key: string) {
    return await this.store.get(key);
  }

  async setAppStore(key: string, value: any) {
    return await this.store.set(key, value);
  }

  async listen_event() {
    return await listen("cpcp", (event) => {
      console.log("Event received", event);
      const store = useTranslateStore();
      store.sourceInputFromClipboard = event.payload;
    });
  }

  async on_ready() {
    getCurrent().emit("on_ready", {});
  }
}

export default defineNuxtPlugin({
  name: "tauri",
  async setup(app) {
    const tauri = new Tauri();
    tauri.listen_event();
    app.provide("tauri", tauri);
  },
  hooks: {
    "app:created"() {
      console.log("App created");
      const { $tauri } = useNuxtApp();
      $tauri.on_ready();
      const isDev = process.env.NODE_ENV === "development";
      if (isDev) {
        console.log("Development mode");
      } else {
        console.log("Production mode");
      }
    },
  },
});
