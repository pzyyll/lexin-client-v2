<script setup lang="ts">
import { useKeyModifier } from "@vueuse/core";

interface Option {
  name: string;
  icon: any;
}

const selected = defineModel<Array<Option>>();
const props = defineProps<{
  options?: Array<Option>;
}>();
const ctrlState = useKeyModifier("Control");

watch(selected, (v, pre) => {
  //check length now.value
  if (v.length <= 0) {
    selected.value = pre;
    return;
  }
  if (!ctrlState.value && v.length > 1) {
    // if pre collection includes v collection, skip
    // if v includes pre, only keep increase one
    const subv = v.filter((item) => !pre.includes(item));
    console.log(subv);
    if (subv.length >= 1) {
      selected.value = subv;
      return;
    }
  }
});
</script>

<template>
  <USelectMenu
    v-model="selected"
    :options="props.options"
    :uiMenu="{ width: 'w-auto' }"
    :popper="{ placement: 'top-start' }"
    option-attribuite="name"
    class="unwrapped"
    multiple
  >
    <div class="flex items-center">
      <button
        class="flex items-center gap-1 du-btn du-btn-circle min-h-0 min-w-4 h-6 w-auto px-1"
      >
        <SvgsIcon v-if="selected?.length == 1" :name="selected[0].icon" class="size-4" />
        <icon-clarity-ellipsis-horizontal-line v-else class="size-4" />
        <!-- <component v-else v-for="item in selected" :is="item.icon" class="size-4"></component> -->
      </button>
    </div>
    <template #option="{ option }">
      <div class="flex items-center gap-1">
        <SvgsIcon :name="option.icon" class="size-4" />
        <p>{{ option.name }}</p>
      </div>
    </template>
  </USelectMenu>
</template>
<style scoped>
.unwrapped p {
  @apply whitespace-nowrap;
}
</style>
