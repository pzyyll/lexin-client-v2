<script setup lang="ts">
import { getCurrentWindow as getCurrent } from "@tauri-apps/api/window";

let is_maximized = ref(false);
let is_pin = ref(false);

const appWindow = getCurrent();

const onToggleMaximize = () => {
  appWindow.toggleMaximize().then(() => {
    appWindow.isMaximized().then((maximized) => {
      is_maximized.value = maximized;
      console.log("is_maximized", is_maximized.value);
    });
  });
};

const onTogglePin = () => {
  is_pin.value = !is_pin.value;
  // appWindow.setAlwaysOnTop(is_pin.value);
};
</script>
<template>
  <div data-tauri-drag-region class="flex relative h-8 bg-neutral bg-opacity-90">
    <div id="titlebar-title" class="flex absolute inset-y-0 left-2">
      <icon-svgs-lnb />
      <span class="ml-2 select-none">{{ $t("app_title") }}</span>
    </div>
    <div class="flex absolute right-0 inset-y-0 items-center h-full">
      <button
        class="du-btn du-btn-ghost rounded-none h-full min-h-0 no-app-drag"
        @click="onTogglePin"
      >
        <icon-clarity-pin-line v-if="!is_pin" />
        <icon-clarity-pin-solid v-else />
      </button>
      <button
        class="du-btn du-btn-ghost rounded-none h-full min-h-0 select-none no-app-drag"
        id="titlebar-minimize"
        @click="appWindow.minimize"
      >
        <icon-clarity-minus-line />
      </button>
      <button
        class="du-btn du-btn-ghost rounded-none h-full min-h-0 select-none no-app-drag"
        id="titlebar-maximize"
        @click="onToggleMaximize"
      >
        <icon-clarity-window-max-line v-if="!is_maximized" />
        <icon-clarity-window-restore-line v-else />
      </button>
      <button
        class="du-btn du-btn-ghost rounded-none h-full min-h-0 hover:bg-red-600 hover:bg-opacity-85 select-none no-app-drag"
        id="titlebar-close"
        @click="appWindow.close"
      >
        <icon-clarity-close-line />
      </button>
    </div>
  </div>
</template>

<style scoped>
.no-app-drag {
  -webkit-app-region: no-drag;
}
</style>
