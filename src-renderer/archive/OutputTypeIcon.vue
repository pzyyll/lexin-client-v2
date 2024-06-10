<script setup lang="ts">
const { $translateType } = useNuxtApp();
const opts: Array<Translate.TranslateTypeInfo> = Object.values(
  $translateType as Object
).sort((a, b) => a.sort - b.sort);

const options = ref(opts);
const selected = ref([options.value[0]]);

const languages = [
  { name: "中文", code: "zh" },
  { name: "English", code: "en" },
  { name: "Français", code: "fr" },
  { name: "Deutsch", code: "de" },
  { name: "日本語", code: "ja" },
  { name: "한국어", code: "ko" },
  { name: "Español", code: "es" },
  { name: "Italiano", code: "it" },
  { name: "Nederlands", code: "nl" },
  { name: "Polski", code: "pl" },
  { name: "Português", code: "pt" },
  { name: "Русский", code: "ru" },
  { name: "Türkçe", code: "tr" },
  { name: "العربية", code: "ar" },
  { name: "हिन्दी", code: "hi" },
  { name: "ไทย", code: "th" },
  { name: "Tiếng Việt", code: "vi" },
  { name: "Bahasa Indonesia", code: "id" },
  { name: "Melayu", code: "ms" },
  { name: "Filipino", code: "fil" },
  { name: "Svenska", code: "sv" },
  { name: "Norsk", code: "no" },
  { name: "Dansk", code: "da" },
  { name: "Suomi", code: "fi" },
  { name: "Čeština", code: "cs" },
  { name: "Slovenčina", code: "sk" },
  { name: "Română", code: "ro" },
  { name: "Magyar", code: "hu" },
  { name: "Pусский", code: "ru" },
  { name: "Українська", code: "uk" },
  { name: "Беларуская", code: "be" },
  { name: "Български", code: "bg" },
  { name: "Ελληνικά", code: "el" },
  { name: "Türkçe", code: "tr" },
];

const sourceLang = ref([languages[0]]);
const targetLang = ref([languages[0]]);
</script>
<template>
  <div class="flex bg-gray-100 rounded-2xl shadow-md">
    <TranslateTypeSelect :options="options" v-model="selected" />
    <div class="flex items-center pl-1 pr-2">
      <TranslateLangSelect :options="languages" v-model="sourceLang" />
      <div class="du-btn du-btn-ghost min-h-0 min-w-0 size-4 hover:bg-transparent p-0">
        <icon-heroicons-solid-switch-horizontal class="size-4" />
      </div>
      <TranslateLangSelect :options="languages" v-model="targetLang" />
    </div>
  </div>
</template>
