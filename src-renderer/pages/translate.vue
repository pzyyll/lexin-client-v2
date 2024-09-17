<script setup lang="ts">
definePageMeta({
  layout: "notstyle",
});
import { vElementSize } from "@vueuse/components";

const { $media, $tauri } = useNuxtApp();
const { snapEdgeSize } = useAppConfig();

const lg = $media.lg();
const container = ref<HTMLElement | null>(null);
const { width: containerWidth, height: containerHeight } = useElementSize(container);
const toolsbarIsOver = ref(false);

const snapEdge = computed(() => {
  return containerWidth.value < snapEdgeSize;
});

const snapEdgeStyleClass = computed(() => {
  if (snapEdge.value) {
    return "px-0 py-0";
  }
  return "px-2 pb-2";
});
const cardSnapEdgeStyleClass = computed(() => {
  if (snapEdge.value) {
    return "rounded-none shadow-none";
  }
  return "";
});

const onResize = ({ width, height }: { width: number; height: number }) => {
  nextTick(async () => {
    await $tauri.onTranslateInputResize({ width, height });
  });
};

// watch(containerWidth, () => {
//   console.log("containerWidth", containerWidth.value);
// });
</script>

<template>
  <div
    class="flex flex-col h-full"
    :class="{
      relative: snapEdge,
    }"
  >
    <TranslateToolsbar
      v-model="toolsbarIsOver"
      class="flex-none min-h-1"
      :class="{
        'absolute top-0 z-10': snapEdge && !toolsbarIsOver,
      }"
    />
    <div
      class="flex-1 flex flex-col lg:flex-row lg:pt-1 overflow-y-auto"
      :class="[
        snapEdgeStyleClass,
        {
          'z-0': snapEdge,
        },
      ]"
      v-element-size="onResize"
      ref="container"
    >
      <div
        id="translate-input"
        class="flex flex-col flex-none sticky top-0 w-auto max-h-[46svh] lg:static lg:flex-1 lg:h-full lg:max-h-full z-10"
      >
        <TranslateInput class="w-auto h-full p-2 rou" :class="cardSnapEdgeStyleClass" />
      </div>
      <div
        v-if="!lg && !snapEdge"
        class="flex-initial du-divider h-2.5 mt-0 mb-0 px-4"
      ></div>
      <div
        v-if="lg"
        class="flex-initial du-divider du-divider-horizontal mx-0 py-4"
      ></div>
      <div
        id="translate-output"
        class="bg-transparent flex flex-1 flex-col w-auto h-full lg:max-h-full overflow-y-auto gap-2"
      >
        <!-- <p>{{ translateInput }}</p> -->
        <TranslateOutputTabs class="w-auto p-2" :class="cardSnapEdgeStyleClass" />
      </div>
    </div>
  </div>
</template>
