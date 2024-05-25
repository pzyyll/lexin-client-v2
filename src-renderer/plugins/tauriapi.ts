import { WebviewWindow } from "@tauri-apps/api/webviewWindow";

export class Tauri {
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
}

export default defineNuxtPlugin({
  name: "tauri",
  async setup(app) {
    const tauri = new Tauri();
    app.provide("tauri", tauri);
  },
  hooks: {
    "app:created"() {
      console.log("App created");
      const isDev = process.env.NODE_ENV === "development";
      if (isDev) {
        console.log("Development mode");
      } else {
        console.log("Production mode");
      }
    },
  },
});
