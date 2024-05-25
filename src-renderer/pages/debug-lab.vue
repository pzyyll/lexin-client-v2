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

</script>

<template>
  <div>
    <!-- <MarkdownText Value="# Lab Dashboard" /> -->
    <br />
    <div class="flex flex-row gap-2">
      <button class="daisy-btn" @click="openPlayground">Playground</button>
    </div>
  </div>
</template>
