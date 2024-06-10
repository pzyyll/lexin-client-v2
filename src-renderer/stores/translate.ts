export const useTranslateStore = defineStore("translate", () => {
  const selectedTranslateTypes = ref<Translate.TranslateTypeInfo[]>([]);
  const selectedTranslateTab = ref<Translate.TranslateTypeInfo>({
    name: "",
    icon: "",
    sort: 0,
    api_type: "",
  });
  const sourceLanguage = ref<string>("");
  const targetLanguage = ref<string>("");
  const sourceDetectLanguage = ref<string>("");
  const sourceInput = ref<string>("");
  const sourceInputFromClipboard = ref<string>("");
  const isSourceInputing = ref<boolean>(false);
  const isTranslating = ref<boolean>(false);
  const outputTabTranslateText: { [key: string]: string } = reactive({});
  const outputTabShowText: { [key: string]: string } = reactive({});

  const targetDetectLanguage = computed(() => {
    if (targetLanguage.value) return targetLanguage.value;
    if (sourceDetectLanguage.value && sourceDetectLanguage.value.startsWith("zh"))
      return "en";
    return "zh";
  });

  const getTabTranslateText = computed(() => {
    return (tabname: string) => {
      if (!(tabname in outputTabTranslateText)) {
        outputTabTranslateText[tabname] = "";
      }
      return outputTabTranslateText[tabname];
    };
  });

  const setTabTranslateText = (tabname: string, text: string) => {
    outputTabTranslateText[tabname] = text;
  };

  const getTabShowText = computed(() => {
    return (tabname: string) => {
      if (!(tabname in outputTabShowText)) {
        outputTabShowText[tabname] = "";
      }
      return outputTabShowText[tabname];
    };
  });

  const setTabShowText = (tabname: string, text: string) => {
    outputTabShowText[tabname] = text;
  };
  const setSourceLanguage = (lang: string) => {
    sourceLanguage.value = lang;
  };
  const setTargetLanguage = (lang: string) => {
    targetLanguage.value = lang;
  };
  const setSourceInput = (input: string) => {
    sourceInput.value = input;
  };
  const setSelectedTranslateTypes = (types: Translate.TranslateTypeInfo[]) => {
    selectedTranslateTypes.value = types;
  };
  const setSourceDetectLanguage = (lang: string) => {
    sourceDetectLanguage.value = lang;
  };

  return {
    selectedTranslateTypes,
    selectedTranslateTab,
    sourceLanguage,
    targetLanguage,
    sourceDetectLanguage,
    targetDetectLanguage,
    sourceInput,
    sourceInputFromClipboard,
    isSourceInputing,
    isTranslating,

    setSourceLanguage,
    setTargetLanguage,
    setSelectedTranslateTypes,
    setSourceInput,
    setSourceDetectLanguage,

    getTabTranslateText,
    setTabTranslateText,

    getTabShowText,
    setTabShowText,
  };
});
