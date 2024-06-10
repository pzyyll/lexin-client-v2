<script setup lang="ts">
definePageMeta({
  layout: "container",
});

import { invoke } from "@tauri-apps/api/core";

const { $translate } = useNuxtApp();

const output = ref<string>("Output...");

const translate_text = async () => {
  try {
    const result = await invoke<string>("translate_text", {
      apiType: "google",
      text: "hello",
      from: "",
      to: "zh",
    });
    output.value = result;
  } catch (error) {
    output.value = error as string;
  }
};

const translate_languages = async () => {
  try {
    const result = await invoke<string>("translate_languages", {
      apiType: "google",
      displayLanguageCode: "zh",
    });
    output.value = result;
  } catch (error) {
    output.value = error as string;
  }
};

const translate_detect = async () => {
  try {
    const result: string = await invoke("translate_detect", {
      apiType: "google",
      text: "hello",
    });
    output.value = result;
  } catch (error) {
    output.value = error as string;
  }
};

const translate_speech = async () => {
  try {
    const result = await $translate.translate_speech({
      text: "application changed files...",
      lang: "en",
    });
    const audio = new Audio(result);
    audio.play();
  } catch (error) {
    output.value = error as string;
  }
};
</script>

<template>
  <div class="flex flex-col gap-2 m-4">
    <div class="flex gap-2">
      <button class="du-btn" @click="translate_text">translate_text</button>
      <button class="du-btn" @click="translate_languages">translate_languages</button>
      <button class="du-btn" @click="translate_detect">translate_detect</button>
      <button class="du-btn" @click="translate_speech">translate_speech</button>
    </div>
    <div class="bg-slate-100 p-2">
      <p>{{ output }}</p>
    </div>
  </div>
</template>
