<script setup lang="ts">
import { vAutoAnimate } from "@formkit/auto-animate/vue";
const store = useTranslateStore();
const { $translate } = useNuxtApp();

const model = ref<string>();
const edit_mode = ref<boolean>(true);
const ref_text = ref();
const refTextDiv = ref();
const { $media } = useNuxtApp();

const lg = $media.lg();

watchDebounced(
  () => model.value,
  (n, o) => {
    store.setSourceInput(model.value as string);
    store.isSourceInputing = false;
    if (n === "") {
      store.setSourceDetectLanguage("");
    }
  },
  {
    debounce: 500,
    maxWait: 5000,
    onTrigger(event) {
      if (event.newValue) {
        store.isSourceInputing = true;
      }
    },
  }
);

let tid: number | NodeJS.Timeout | undefined = undefined;
const onClickEdit = () => {
  edit_mode.value = true;
  nextTick(() => {
    ref_text.value.focus();

    clearTimeout(tid);
    tid = setTimeout(() => {
      refTextDiv.value.scrollTop = refTextDiv.value.scrollHeight;
    }, 50);
  });
};

const onClickPlay = async (element: any) => {
  console.log("onClickPlay", element);
  if (store.sourceInput === "") return;
  const targetlang = store.sourceLanguage || store.sourceDetectLanguage;
  element.setLoading();
  const src = await $translate.translate_speech({
    text: store.sourceInput,
    lang: targetlang,
  });
  if (element.isLoading()) element.play(src);
};


watch(
  () => store.sourceInputFromClipboard,
  (n, o) => {
    model.value = n;
  }
);

onMounted(() => {
  if (store.sourceInputFromClipboard) {
    model.value = store.sourceInputFromClipboard;
    edit_mode.value = false;
  }
});

</script>

<template>
  <div class="flex flex-col h-auto du-card du-card-bordered rounded-md bg-gray-50">
    <div
      class="du-card-body p-0 max-h-full flex flex-col-reverse lg:flex-col"
      v-auto-animate="{ duration: 200 }"
    >
      <div class="flex-1 overflow-y-auto" ref="refTextDiv">
        <TextAutosize
          v-model="model"
          placeholder="Input here..."
          v-if="edit_mode || lg"
          ref="ref_text"
          class="min-h-16"
          :min-rows="lg ? 0 : 4"
        />
        <div v-if="!edit_mode && !lg">
          <p class="truncate">
            {{ model }}
          </p>
        </div>
      </div>
      <BaseToolsbar>
        <template #start>
          <div class="flex-1 flex items-center gap-1">
            <TranslateTypeIcon />
          </div>
        </template>
        <template #center>
          <TranslateVolume
            class="btn-icon"
            @on-play="onClickPlay"
            v-if="store.sourceInput"
          />
        </template>
        <template #end>
          <div class="flex-none flex flex-row-reverse">
            <Transition name="icon">
              <div
                class="du-btn du-btn-ghost du-btn-circle size-6 min-h-0 min-w-0 lg:hidden"
                @click="edit_mode = !edit_mode"
                v-if="edit_mode"
              >
                <icon-gravity-ui-chevrons-up />
              </div>
            </Transition>
            <div class="btn-icon">
              <icon-gravity-ui-copy />
            </div>
            <div class="btn-icon" @click="onClickEdit">
              <icon-gravity-ui-pencil-to-square />
            </div>
          </div>
        </template>
      </BaseToolsbar>
    </div>
  </div>
</template>

<style scoped>
.icon-enter-active,
.icon-leave-active {
  transition: all 0.1s ease-in-out;
}

.icon-enter-from,
.icon-leave-to {
  opacity: 0;
  width: 0;
}
</style>
