<script setup lang="ts">
import markdownit from "markdown-it";
import highlightjs from "highlight.js";
import "highlight.js/styles/github.css";

const props = defineProps<{
  Content?: string;
}>();

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

const markdownText = computed(() => {
  return props.Content ? md.render(props.Content) : "";
});
</script>

<template>
  <article
    class="prose prose-stone prose-sm md:prose-base leading-snug prose-pre:bg-cool-200 prose-code:text-black"
    v-html="markdownText"
  ></article>
</template>
