<script setup lang="ts">
const input = defineModel<string>();
const props = defineProps<{
  placeholder?: string;
  minRows?: number;
}>();

// const { textarea: edit, input } = useTextareaAutosize();

const edit = ref<HTMLTextAreaElement>();
const root = ref<HTMLElement>();

const textSizes = ["text-base", "text-lg", "text-xl", "text-2xl", "text-3xl"];
const refTextSizeIdx = ref(textSizes.length - 1);
const rcTextSize = computed(() => textSizes[refTextSizeIdx.value]);

const _divDummy = ref<HTMLTextAreaElement>();

const { $rootFontSize } = useNuxtApp();
const minlgMinHeight = computed(() => {
  if (props.minRows) {
    return props.minRows * $rootFontSize;
  }
  return 0;
});

const baseClass = [
  "resize-none",
  "bg-transparent",
  "du-textarea-ghost",
  "px-1",
  "py-0",
  "w-full",
  "h-auto",
  "focus:outline-none",
  "focus:bg-transparent",
  "focus:border-r-0",
];

const textareaClass = computed(() => {
  return [...[rcTextSize.value], ...baseClass, "h-16"];
});

const adjustAutoSize = () => {
  nextTick(() => {
    const el = edit.value!;
    if (!el) return;
    if (el.value === "") {
      el.style.height = "auto";
      el.style.height = root.value!.clientHeight - 8 + "px";
      return;
    }
    el.style.height = "auto";
    el.style.height = el.scrollHeight + "px";
  });
};

const adjustFontSize = (newValue: string, oldValue: string) => {
  if (!_divDummy.value) return;

  const isGrow = newValue.length > oldValue.length;
  const minHeight = minlgMinHeight.value || edit.value!.parentElement!.clientHeight;
  // console.log("minHeight=====================", minHeight, props.minRows);

  const adjustSize = (growCondition: boolean, shrinkCondition: boolean) => {
    let i = refTextSizeIdx.value;

    if (growCondition) {
      for (; i > 0; i--) {
        if (calcTextareaHeight(newValue, textSizes[i])! <= minHeight) break;
      }
    } else if (shrinkCondition) {
      for (i = i + 1; i < textSizes.length; i++) {
        if (calcTextareaHeight(newValue, textSizes[i])! > minHeight) break;
      }
      i--;
    } else if (!newValue) {
      i = textSizes.length - 1;
    }

    if (refTextSizeIdx.value != i) {
      refTextSizeIdx.value = i;
    }
  };

  adjustSize(
    isGrow && edit.value!.scrollHeight >= minHeight,
    !isGrow && edit.value!.scrollHeight <= minHeight
  );

  adjustAutoSize();
};

const adjustFontSizeFoce = () => {
  const minHeight = minlgMinHeight.value || edit.value!.parentElement!.clientHeight;
  if (input.value && input.value !== "") {
    let i = textSizes.length - 1;
    for (; i > 0; i--) {
      const fh = calcTextareaHeight(input.value, textSizes[i])!;
      if (fh <= minHeight) {
        break;
      }
    }
    refTextSizeIdx.value = i;
  }
  // adjustAutoSize();
};

const calcTextareaHeight = (text: string, size: string) => {
  if (!_divDummy.value || !edit.value) return;
  _divDummy.value.className = `${size}`;
  const style = getComputedStyle(edit.value);
  _divDummy.value.style.width = style.width;
  _divDummy.value.style.padding = style.padding;
  _divDummy.value.style.border = style.border;
  _divDummy.value.style.boxSizing = style.boxSizing;

  _divDummy.value.textContent = text;

  const height = _divDummy.value.scrollHeight;
  // console.log("_divDummy height=====================", height, size, text);

  return height;
};

const focus = () => {
  edit.value!.focus();
  // setTimeout(() => {
  //   console.log("root.value!.scrollHeight", root.value!.scrollHeight);
  //   root.value!.parentElement!.scrollTop = root.value!.scrollHeight;
  // }, 50);
};

useResizeObserver(root, () => {
  if (input.value) {
    adjustFontSizeFoce();
  }
  adjustAutoSize();
});

watch(input, async (n, o) => {
  const newValue = n || "";
  const oldValue = o || "";
  // console.log("adjust font size=====================", newValue, oldValue);
  adjustAutoSize();
  nextTick(() => adjustFontSize(newValue, oldValue));
});

const emit = defineEmits<{
  onPaste: [e: ClipboardEvent];
}>();

const onPaste = (e: ClipboardEvent) => {
  emit("onPaste", e);
};

onMounted(() => {
  adjustAutoSize();
  if (input.value) {
    adjustFontSizeFoce();
    // setTimeout(() => {
    //   focus();
    // }, 200);
  }
});

defineExpose({
  focus,
});
</script>

<template>
  <div class="w-full h-full relative" ref="root">
    <textarea
      :class="textareaClass"
      v-model="input"
      :placeholder="props.placeholder"
      ref="edit"
      @paste="onPaste"
    >
    </textarea>
    <!-- <textarea class="hidden-element min-h-0" :class="baseClass" ref="_refFake"></textarea> -->
    <div id="_dummy" ref="_divDummy"></div>
  </div>
</template>

<style scoped>
textarea {
  -ms-overflow-style: none;
  scrollbar-width: none;
  overflow: hidden;
}

textarea::-webkit-scrollbar {
  display: none;
}

/* .hidden-element {
  position: absolute;
  top: -9999px;
  visibility: hidden;
  height: auto;
  overflow: hidden;
  min-height: 0px;
} */

#_dummy {
  position: absolute;
  top: -9999px;
  visibility: hidden;
  height: auto;
  overflow: hidden;

  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>
