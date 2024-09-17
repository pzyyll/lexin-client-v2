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
  $translate
    .translate_speech({
      text: store.sourceInput,
      lang: targetlang,
    })
    .then((src) => {
      if (element.isLoading()) element.play(src);
    })
    .catch((err) => {
      console.error(err);
      element.reset();
    });
};

watch(
  () => store.sourceInputFromClipboard,
  (n, o) => {
    model.value = n;
  }
);

const imgs = ref<string[]>([]);
const onPaste = (e: ClipboardEvent) => {
  const items = e.clipboardData?.items;
  if (!items) return;
  if (items.length === 0) return;
  const item = items[0];
  if (item.type.startsWith("image")) {
    console.log("image ", item);
    const blob = item.getAsFile();

    const src = URL.createObjectURL(blob as Blob);
    imgs.value.push(src);

    blob?.arrayBuffer().then((buffer) => {
      console.log("buffer", buffer);
      $translate
        .translate_img2text({ img: buffer })
        .then((res) => {
          model.value = res.detected_text;
        })
        .catch((err) => {
          console.error(err);
        });
    });

    // const reader = new FileReader();
    // reader.onload = (e) => {
    //   const data = e.target?.result;
    //   console.log("data", data);

    //   // imgs.value.push(data as string);
    //   // $translate
    //   //   .translate_img2text({ img: data as string })
    //   //   .then((res) => {
    //   //     console.log("res", res);
    //   //   })
    //   //   .catch((err) => {
    //   //     console.error(err);
    //   //   });
    // };
    // reader.readAsArrayBuffer(blob!);
  }
};

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
      <div class="flex-1 flex overflow-y-auto flex-col relative" ref="refTextDiv">
        <div
          v-if="imgs.length"
          class="flex shrink-0 gap-2 overflow-x-auto sticky top-0 z-10 bg-gray-50"
        >
          <img v-for="img in imgs" :src="img" alt="image" class="h-6 w-6 rounded" />
          <button class="btn-icon" @click="imgs.length = 0">
            <icon-gravity-ui-square-dashed-text />
          </button>
        </div>
        <div class="flex-none z-0">
          <TextAutosize
            v-model="model"
            placeholder="Input here..."
            v-if="edit_mode || lg"
            ref="ref_text"
            class="min-h-16"
            :min-rows="lg ? 0 : 4"
            @on-paste="onPaste"
          />
          <div v-if="!edit_mode && !lg">
            <p class="truncate">
              {{ model }}
            </p>
          </div>
        </div>
      </div>
      <BaseToolsbar class="flex-none">
        <template #start>
          <div class="flex-1 flex items-center gap-1">
            <TranslateTypeIcon />
          </div>
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
            <TranslateVolume
              class="btn-icon"
              @on-play="onClickPlay"
              v-if="store.sourceInput"
            />
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
