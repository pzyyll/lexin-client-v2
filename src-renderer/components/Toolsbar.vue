<script setup lang="ts">

const isOver = defineModel<boolean>();

const el = ref();
const isPin = ref(false);
// const isOver = ref(false);

const onMouseOver = (event: MouseEvent) => {
  isOver.value = true;
};

const onMouseOut = () => {
  isOver.value = false;
};

watch(isPin, (value) => {
  console.log("isPin", value);
});
</script>

<template>
  <div
    @mouseenter="onMouseOver"
    @mouseleave="onMouseOut"
    :class="{
      'h-1': !isOver,
      'bg-secondary0': !isOver,
      'h-8': isOver,
      'w-8': !isOver,
    }"
    class="content-center transition-[height] duration-100 ease-in-out mb-1 rounded-b-xl ml-2.5"
    ref="el"
  >
    <div class="flex h-6 rounded-b-xl transition duration-100" v-if="isOver">
      <div
        class="du-btn du-btn-ghost du-btn-circle h-full size-6 min-w-0 min-h-0 p-0"
        @click="isPin = !isPin"
      >
        <icon-clarity-pin-line v-if="!isPin" class="-scale-x-100" />
        <icon-clarity-pin-solid v-else class="-scale-x-100" />
      </div>
    </div>
  </div>
</template>
