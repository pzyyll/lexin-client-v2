import { Tauri } from "~/plugins/tauriapi";

declare module "#app" {
  interface NuxtApp {
    $tauri: Tauri;
  }
}

export {};
