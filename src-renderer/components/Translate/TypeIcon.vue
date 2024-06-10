<script setup lang="ts">
const { $translateType, $translate } = useNuxtApp();
const options: Array<Translate.TranslateTypeInfo> = Object.values(
  $translateType as Object
).sort((a, b) => a.sort - b.sort);

const { default_language } = useAppConfig();
const store = useTranslateStore();

const languages = ref([{ name: "Auto", code: "" }]);
const sourceLang = ref(
  languages.value.find((lang) => lang.code === store.sourceLanguage) || languages.value[0]
);
const targetLang = ref(
  languages.value.find((lang) => lang.code === store.targetLanguage) || languages.value[0]
);

const sourceExtra = computed(() => {
  if (!sourceLang.value.code && store.sourceDetectLanguage) {
    return store.sourceDetectLanguage.toLowerCase();
  }
  return "";
});

const targetExtra = computed(() => {
  if (!targetLang.value.code && store.sourceDetectLanguage) {
    return store.sourceDetectLanguage.startsWith("zh") ? "en" : "zh";
  }
  return "";
});

const loadLanguages = async () => {
  // fetch languages
  const result = await $translate.translate_languages({
    apiType: "google",
    displayLanguageCode: default_language,
  });

  const langs = [{ name: "Auto", code: "" }];
  result.languages.forEach((lang) => {
    langs.push({ name: lang.display_name, code: lang.language_code });
  });
  languages.value = langs;
};

watch([sourceLang, targetLang], ([source, target]) => {
  store.setSourceLanguage(source.code);
  store.setTargetLanguage(target.code);
});

onMounted(() => {
  loadLanguages();
  if (store.selectedTranslateTypes!.length === 0) {
    store.setSelectedTranslateTypes([options[0]]);
  }
});
</script>
<template>
  <div class="flex bg-gray-100 rounded-2xl shadow-md">
    <TranslateTypeSelect :options="options" v-model="store.selectedTranslateTypes" />
    <div class="flex items-center pl-1 pr-2">
      <TranslateLangSelect
        :options="languages"
        :extra="sourceExtra"
        v-model="sourceLang"
      />
      <div class="du-btn du-btn-ghost min-h-0 min-w-0 size-4 hover:bg-transparent p-0">
        <icon-heroicons-solid-switch-horizontal class="size-4" />
      </div>
      <TranslateLangSelect
        :options="languages"
        :extra="targetExtra"
        v-model="targetLang"
      />
    </div>
  </div>
</template>
