<template>
  <div data-tauri-drag-region class="bg-red-300">
    <div id="titlebar-minimize">
      <icon-mdi-window-minimize />
    </div>
    <div id="titlebar-maximize">
      <icon-mdi-window-maximize />
    </div>
    <div id="titlebar-close">
      <icon-mdi-close />
    </div>
  </div>
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>
