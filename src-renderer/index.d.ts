import translate from "~/pages/translate.vue";

declare global {
  declare namespace Translate {
    declare interface TranslateTypeInfo {
      name: string;
      icon: any;
      sort: number;
      api_type: string;
    };
    declare type TranslateType = {
        [key: string]: TranslateTypeInfo;
    };
  }
}

declare module 'nuxt/schema' {
  interface AppConfigInput {
    /** Theme configuration */
    title?: string;
    snapEdgeSize: number;
    default_language: string;
  }
}

export {};
