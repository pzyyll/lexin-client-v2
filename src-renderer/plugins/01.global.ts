import { useMediaQuery } from "@vueuse/core";

export default defineNuxtPlugin((nuxtapp) => {
  const media = {
    sm: () => useMediaQuery("(min-width: 640px)"),
    md: () => useMediaQuery("(min-width: 768px)"),
    lg: () => useMediaQuery("(min-width: 1024px)"),
    xl: () => useMediaQuery("(min-width: 1280px)"),
    xxl: () => useMediaQuery("(min-width: 1536px)"),
    maxsm: () => useMediaQuery("(max-width: 640px)"),
    maxmd: () => useMediaQuery("(max-width: 768px)"),
    maxlg: () => useMediaQuery("(max-width: 1024px)"),
    maxxl: () => useMediaQuery("(max-width: 1280px)"),
    maxxxl: () => useMediaQuery("(max-width: 1536px)"),
  };
  nuxtapp.provide("media", media);

  const translateType = {
    google: { name: "Google", icon: "IconGoogle", sort: 1, api_type: "google" },
    deepl: { name: "DeepL", icon: "IconDeepL", sort: 2, api_type: "deepl" },
    baidu: { name: "Baidu", icon: "IconBaidu", sort: 3, api_type: "baidu" },
    chatgpt: {
      name: "ChatGPT",
      icon: "IconChatGpt",
      sort: 4,
      api_type: "chatgpt",
    },
  };
  nuxtapp.provide("translateType", translateType);

  nuxtapp.hook("app:mounted", () => {
    nuxtapp.provide(
      "rootFontSize",
      parseFloat(getComputedStyle(document.documentElement).fontSize)
    );
  });
});
