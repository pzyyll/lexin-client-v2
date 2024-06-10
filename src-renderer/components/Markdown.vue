<script setup lang="ts">
defineOptions({
  inheritAttrs: false,
});

import markdownit from "markdown-it";
import highlightjs from "highlight.js";
import "highlight.js/styles/github.css";
import { _0 } from "#tailwind-config/theme/backdropBlur";

const props = defineProps<{
  Content?: string;
  ShowTextLoading?: boolean;
  FontAdapter?: boolean;
  FontThreshold?: number;
  Placeholder?: string;
}>();
const parser = new DOMParser();

const md = markdownit({
  html: false,
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

const fontSizes = [
  "prose-sm",
  "prose-base",
  "prose-lg",
  "prose-xl",
  "prose-2xl",
  "text-3xl",
];
const rootElement = ref<HTMLElement | null>(null);
const fontSizeIdx = ref(fontSizes.length - 1);
const fakeElement = ref<HTMLElement | null>(null);

const rcFontSize = computed(() => fontSizes[fontSizeIdx.value]);
const fontThreshold = computed(() => {
  return props.FontThreshold ? props.FontThreshold * $rootFontSize : 0;
});

const { $rootFontSize } = useNuxtApp();

const textAfter = (parent: HTMLElement) => {
  function findTheLastNode(parent: HTMLElement) {
    if (parent.childNodes.length === 0) {
      return parent;
    }
    const nodes = parent.childNodes;
    let lastTextNode: HTMLElement | Node | null = null;
    for (let i = nodes.length - 1; i >= 0; i--) {
      if (nodes[i].nodeType === Node.TEXT_NODE && nodes[i].textContent?.trim() != "") {
        lastTextNode = nodes[i];
        break;
      } else if (nodes[i].nodeType === Node.ELEMENT_NODE) {
        lastTextNode = findTheLastNode(nodes[i] as HTMLElement) as HTMLElement;
        break;
      }
    }
    return lastTextNode;
  }
  function getFirstParentElement(node: Node) {
    if (node.nodeType === Node.ELEMENT_NODE) {
      return node as HTMLElement;
    }
    return getFirstParentElement(node.parentNode as Node);
  }

  const lastTextNode = findTheLastNode(parent);

  if (lastTextNode) {
    if (lastTextNode.nodeType === Node.TEXT_NODE) {
      getFirstParentElement(lastTextNode).classList.add("p-after-loading");
    } else {
      (lastTextNode as HTMLElement).classList.add("p-after-loading");
    }
  }
};

const markdownText = computed(() => {
  let content = "";
  if (props.Content) {
    content = props.Content;
  } else if (props.Placeholder && !props.ShowTextLoading) {
    content = props.Placeholder;
  } else {
    return "";
  }
  const html = md.render(content);
  const doc = parser.parseFromString(html, "text/html");
  if (props.ShowTextLoading) {
    textAfter(doc.body);
  }
  return doc.body.innerHTML;
});

const isShowPlaceholder = computed(() => {
  return !props.Content && props.Placeholder && !props.ShowTextLoading;
});

watch(markdownText, (newValue, oldValue) => {
  autoFontSize(newValue, oldValue);
});

const getFakeHeight = (size: string, text: string) => {
  if (!fakeElement.value) return 0;
  for (let i = 0; i < fakeElement.value!.classList.length; i++) {
    if (fontSizes.includes(fakeElement.value!.classList[i])) {
      fakeElement.value!.classList.remove(fakeElement.value!.classList[i]);
      break;
    }
  }
  fakeElement.value!.className = `${size} ${fakeElement.value!.className}`;
  fakeElement.value!.style.height = "auto";
  fakeElement.value!.innerHTML = text;
  return fakeElement.value!.scrollHeight;
};

const autoFontSize = (newValue: string, oldValue: string) => {
  if (!props.FontAdapter || !rootElement.value || !fakeElement.value) return;
  const minHeight = fontThreshold.value
    ? fontThreshold.value
    : rootElement.value.clientHeight;

  const isGrow = newValue.length > oldValue.length;
  const adjustSize = (growCondition: boolean, shrinkCondition: boolean) => {
    let i = fontSizeIdx.value;

    if (growCondition) {
      for (; i > 0; i--) {
        if (getFakeHeight(fontSizes[i], newValue) <= minHeight) break;
      }
    } else if (shrinkCondition) {
      for (i = i + 1; i < fontSizes.length; i++) {
        if (getFakeHeight(fontSizes[i], newValue) > minHeight) break;
      }
      i--;
    } else if (!newValue) {
      i = fontSizes.length - 1;
    }

    if (fontSizeIdx.value != i) {
      fontSizeIdx.value = i;
    }
  };

  adjustSize(
    isGrow && rootElement.value!.scrollHeight >= minHeight,
    !isGrow && rootElement.value!.scrollHeight <= minHeight
  );
};

const autoFontSizeForce = () => {
  if (!props.FontAdapter || !rootElement.value || !fakeElement.value) return;
  const minHeight = fontThreshold.value
    ? fontThreshold.value
    : rootElement.value.clientHeight;
  // console.log("minHeight=====================", minHeight);
  if (markdownText.value && markdownText.value !== "") {
    let i = 0;
    for (; i < fontSizes.length; i++) {
      if (getFakeHeight(fontSizes[i], markdownText.value) > minHeight) {
        break;
      }
    }
    fontSizeIdx.value = Math.min(fontSizes.length - 1, Math.max(0, i - 1));
  }
};

useResizeObserver(rootElement, () => {
  if (markdownText.value) {
    autoFontSizeForce();
  }
});

onMounted(() => {
  if (markdownText.value) {
    autoFontSizeForce();
  }
});
</script>

<template>
  <div class="overflow-auto relative h-full max-h-full" ref="rootElement">
    <article
      class="prose prose-stone leading-snug prose-pre:bg-cool-200 prose-code:text-black break-words hyphens-auto"
      :class="[
        rcFontSize,
        {
          'text-neutral-400': isShowPlaceholder,
        },
      ]"
      v-html="markdownText"
      v-bind="$attrs"
      placeholder="Translating..."
    ></article>
    <article
      class="prose prose-stone leading-snug prose-pre:bg-cool-200 prose-code:text-black break-words hyphens-auto"
      style="visibility: hidden; position: absolute; top: -9999px; width: 100%"
      ref="fakeElement"
    ></article>
  </div>
</template>

<style scoped>
:deep(.p-after-loading)::after {
  content: "";
  display: block;
  position: relative;
  pointer-events: none;
  top: 0.3em;
  @apply du-loading du-loading-dots du-loading-xs z-0;
}
</style>
