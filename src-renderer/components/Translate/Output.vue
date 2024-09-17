<script setup lang="ts">
import { vAutoAnimate } from "@formkit/auto-animate/vue";
const props = defineProps<{
  tabInfo: Translate.TranslateTypeInfo;
}>();

const { $translate } = useNuxtApp();
const store = useTranslateStore();
const { getTabTranslateText, getTabShowText } = storeToRefs(store);
const isTranslating = ref(false);

const translateText = computed({
  get: () => getTabTranslateText.value(props.tabInfo!.api_type),
  set: (value) => store.setTabTranslateText(props.tabInfo!.api_type, value),
});

const showText = computed({
  get: () => {
    const text = getTabShowText.value(props.tabInfo!.api_type);
    if (text === "" && isTranslating.value) {
      return "Translating";
    } else if (text === "" && store.isSourceInputing) {
      return "Inputing";
    }
    return text;
  },
  set: (value) => store.setTabShowText(props.tabInfo!.api_type, value),
});

const error = ref<string | null>(null);

const reqTranslateText = (text: string) => {
  if (!text.trim()) return;
  console.log("reqTranslateText:", props.tabInfo?.name, "input", text);
  isTranslating.value = true;
  const poyload = {
    apiType: props.tabInfo!.api_type,
    from: store.sourceLanguage,
    to: store.targetLanguage,
    text: text,
  };
  console.log("poyload", poyload);
  $translate
    .translate_text(poyload)
    .then((result) => {
      console.log("Output: translate result ", result);
      error.value = null;
      showText.value = result.text[0];
      isTranslating.value = false;
      result.detected_source_language &&
        store.setSourceDetectLanguage(result.detected_source_language);
    })
    .catch((err) => {
      console.error(err);
      isTranslating.value = false;
      showText.value = "";
      error.value = `${err}`;
    });
};

watch(translateText, (text) => {
  if (!text) {
    showText.value = "";
    return;
  }
  reqTranslateText(text);
});

watch(
  () => store.sourceInput,
  (input) => {
    if (input != translateText.value) {
      translateText.value = input;
    }
  }
);

watch(
  [() => store.sourceLanguage, () => store.targetLanguage],
  ([sourceLang, targetLang]) => {
    if (!store.sourceInput) return;
    showText.value = "";
    if (translateText.value != store.sourceInput) {
      translateText.value = store.sourceInput;
    } else {
      reqTranslateText(store.sourceInput);
    }
  }
);

onMounted(() => {
  console.log("Output mounted", store.sourceInput, translateText.value);
  if (store.sourceInput != translateText.value) {
    translateText.value = store.sourceInput;
  }
});
</script>

<template>
  <div class="flex flex-col max-h-full h-full max-w-full justify-between">
    <Markdown
      :Content="showText"
      :ShowTextLoading="store.isSourceInputing || isTranslating"
      class="max-w-full"
      :FontAdapter="true"
      Placeholder="Translate..."
    />
    <div class="flex-none" v-if="error" v-auto-animate>
      <p>Error</p>
    </div>
  </div>
</template>
