<script setup lang="ts">
import TranslateOutput from "@/components/Translate/Output.vue";

const store = useTranslateStore();

const { $translate } = useNuxtApp();
const { selectedTranslateTypes: tabs, selectedTranslateTab } = storeToRefs(store);

const selectedTab = ref(0);

const changeTab = (index: number) => {
  selectedTab.value = index;
  selectedTranslateTab.value = tabs.value[index];
};

const showVolume = computed(() => {
  const curApiType = selectedTranslateTab.value.api_type;
  const curText = store.getTabShowText(curApiType);
  return curText && curText.length > 0;
});

const onClickPlay = (el: any) => {
  const curApiType = selectedTranslateTab.value.api_type;
  const curText = store.getTabShowText(curApiType);

  if (!curText) return;
  el.setLoading();
  $translate
    .translate_speech({
      text: curText,
      lang: store.targetDetectLanguage,
    })
    .then((src) => {
      if (el.isLoading()) el.play(src);
    });
};

onMounted(() => {
  selectedTranslateTab.value = tabs.value[selectedTab.value];
});
</script>

<template>
  <div
    class="flex flex-col h-full du-card du-card-bordered rounded-md shadow-xl bg-stone-100"
  >
    <div
      class="du-card-body p-0 flex justify-between max-h-full"
      v-auto-animate="{ duration: 200 }"
    >
      <HTabGroup :selectedIndex="selectedTab" @change="changeTab">
        <HTabPanels class="flex-1 overflow-auto">
          <HTabPanel v-for="tab in tabs" :key="tab.api_type" class="max-h-full h-full">
            <TranslateOutput :tabInfo="tab" />
          </HTabPanel>
        </HTabPanels>
        <BaseToolsbar class="flex-none">
          <template #start>
            <HTabList class="flex flex-none gap-2" v-if="tabs.length > 1">
              <HTab
                as="template"
                v-for="tab in tabs"
                :key="tab.sort"
                v-slot="{ selected }"
              >
                <button
                  class="du-btn du-btn-circle min-h-0 min-w-0 size-6 outline-none"
                  :class="{ 'du-btn-active': selected }"
                >
                  <SvgsIcon :name="tab.icon" class="size-4" />
                </button>
              </HTab>
            </HTabList>
          </template>
          <template #center>
            <div class="flex gap-1">
              <TranslateVolume
                class="btn-icon"
                @on-play="onClickPlay"
                v-if="showVolume"
              />
            </div>
          </template>
          <template #end>
            <div class="du-btn du-btn-ghost du-btn-circle size-6 min-h-0 min-w-0">
              <icon-gravity-ui-copy />
            </div>
          </template>
        </BaseToolsbar>
      </HTabGroup>
    </div>
  </div>
</template>
