<script setup lang="ts">
const { $tauri } = useNuxtApp();

async function openPlayground() {
    const url = "playground"
    console.log("openPlayground: ", url);
    console.log("import.meta.url: ", import.meta.url);
    console.log("router location: ", window.location.pathname);
    console.log("process.env", process.env);
    const webviewWindow = await $tauri.openPlayground();
}

function openTranslate() {
    console.log("openTranslate");
    $tauri.openTranslateWindow();
}

</script>

<template>
  <div>
    <!-- <MarkdownText Value="# Lab Dashboard" /> -->
    <br />
    <div class="flex flex-row gap-2">
      <button class="du-btn" @click="openPlayground">Playground</button>
      <button class="du-btn" @click="openTranslate">Translate</button>
    </div>
  </div>
</template>
