import { Tauri } from "~/plugins/02.tauriapi";
import { Translate } from "~/plugins/03.translate";

declare module "#app" {
  interface Media {
    sm: () => ref<boolean>;
    md: () => ref<boolean>;
    lg: () => ref<boolean>;
    xl: () => ref<boolean>;
    xxl: () => ref<boolean>;
    maxsm: () => ref<boolean>;
    maxmd: () => ref<boolean>;
    maxlg: () => ref<boolean>;
    maxxl: () => ref<boolean>;
    maxxxl: () => ref<boolean>;
  }

  interface NuxtApp {
    $tauri: Tauri;
    $media: Media;
    $rootFontSize: number;
    $translate: Translate;
    $translateType: Translate.TranslateType;
  }
}

export {};
