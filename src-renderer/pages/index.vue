<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
import markdownit from "markdown-it";
import highlightjs from "highlight.js";
import "highlight.js/styles/github.css";

const md = markdownit({
  html: true,
  breaks: true,
  linkify: true,
  typographer: true,
  highlight: function (str, lang) {
    if (lang && highlightjs.getLanguage(lang)) {
      try {
        return highlightjs.highlight(str, { language: lang, ignoreIllegals: true }).value;
      } catch (__) {}
    }
    return ""; // use external default escaping
  },
});

async function openDialog() {
  const result = await invoke("test_cmd", { router: "deepl" });
  console.log(result);
}

async function openAbout() {
  console.log("openAbout");
  const result = await invoke("open_window", { router: "about" });
  console.log(result);
}

const markdown = ref<string>(`
# Hello Worldl
This is a **markdown** text.
and this is a [link](https://www.google.com)
~~~javascript
console.log("Hello World");
~~~
## Title##
### Title
---
- [ ] Task 1
- [x] Task 2
`);

const markdownText = computed(() => {
  const value = md.render(markdown.value);
  // console.log(value);
  return value;
});

onMounted(() => {
  register("CmdOrCtrl+Alt+D", (event) => {
    console.log("CmdOrCtrl+Alt+D", event.state);
    if (event.state === "Pressed") openAbout();
  });
});

onUnmounted(() => {
  unregister("CmdOrCtrl+Alt+D");
});
</script>

<template>
  <div>
    <article class="prose">
      <h1>Sphinx of black quartz, judge my vow.</h1>
      <h2>QWERTYUIOPASDFGHJKLZXCVBNM1234567890</h2>
    </article>
    <h1>Welcome to the homepage QWERTYUIOPASDFGHJKLZXCVBNM1234567890</h1>
    <h6>QWERTYUIOPASDFGHJKLZXCVBNM1234567890</h6>
    <Alert> This is an auto-imported component. </Alert>
    <icon-mdi-pin-off />
    <icon-mdi-pin-off />
    <icon-mdi-google-translate />
    <icon-svgs-deepl-logo />
    <button class="daisy-btn"><icon-svgs-deepl-logo /></button>
  </div>
  <article class="prose prose-slate" v-html="markdown"></article>
  <UButton loading>Button</UButton>
  <!-- <progress class="progress"></progress> -->
  <div class="flex flex-row">
    <div class="basis-1/4 bg-lime-700">01</div>
    <div class="basis-1/4 bg-stone-400">02</div>
    <div class="basis-1/2 bg-orange-200">03</div>
  </div>
  <button class="daisy-btn daisy-btn-square daisy-btn-outline" @click="openAbout">OpenDialog</button>

  <div class="daisy-divider"></div>
  <UTextarea v-model="markdown" autoresize />
  <UDivider />
  <MarkdownText :Value="markdown" />
  <UDivider />
  <div class="h-32 overflow-y-scroll">
    <div class="h-64 bg-slate-400"></div>
  </div>

  <div
    div
    class="h-32 overflow-y-scroll \ scrollbar-thin \ scrollbar-thumb-slate-300 \ scrollbar-track-slate-100 \ hover:scrollbar-thumb-slate-400 \ active:scrollbar-thumb-slate-200"
  >
    <div class="h-64 bg-slate-400"></div>
  </div>

  <div
    class="scrollbar-w-4 scrollbar-thumb-slate-700 scrollbar-track-slate-300 h-32 overflow-y-scroll"
  >
    <div class="h-64 bg-slate-400"></div>
  </div>
  <div
    class="hover:scrollbar-thumb-sky-500 active:scrollbar-thumb-sky-400 h-32 scrollbar scrollbar-thumb-slate-700 scrollbar-track-slate-300 overflow-y-scroll"
  >
    <div class="h-64 bg-slate-400"></div>
  </div>
  <div class="h-32">
    <div class="h-64 bg-red-300">
    </div>
  </div>
</template>

<style scoped>
.scrollbar-thin-custom {
  scrollbar-width: thin;
}
</style>
