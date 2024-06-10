<script setup lang="ts">
import type { FunctionalComponent, SVGAttributes } from "vue";

import IconGoogle from "~icons/svgs/google-translate-logo";
import IconDeepL from "~icons/svgs/deepl-logo";
import IconBaidu from "~icons/svgs/baidu-logo";
import IconChatGpt from "~icons/svgs/chatgpt-logo";

const props = defineProps<{
  name: string;
}>();

const components: Record<string, FunctionalComponent<SVGAttributes>> = {
  IconGoogle,
  IconDeepL,
  IconBaidu,
  IconChatGpt,
};
</script>

<template>
  <component :is="components[props.name]" />
</template>
