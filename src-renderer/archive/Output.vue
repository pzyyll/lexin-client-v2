<script setup lang="ts">
const props = defineProps<{
  text?: string;
}>();

const expand = ref(true);
</script>

<template>
  <div
    class="flex flex-col h-auto du-card du-card-bordered rounded-md shadow-xl bg-stone-100"
  >
    <div class="du-card-body p-0 flex-col-reverse" v-auto-animate="{ duration: 200 }">
      <div class="flex h-6 items-center">
        <div class="flex-1 flex items-center gap-1">
          <TranslateOutputTypeIcon />
          <div
            class="du-btn du-btn-ghost du-btn-circle min-h-0 min-w-0 size-4 hover:bg-transparent"
          >
            <icon-gravity-ui-arrows-rotate-right />
          </div>
        </div>
        <div class="flex-none flex flex-row-reverse">
          <div
            class="du-btn du-btn-ghost du-btn-circle size-6 min-h-0 min-w-0"
            @click="expand = !expand"
          >
            <icon-gravity-ui-chevrons-up
              :class="{ 'rotate-180': expand }"
              class="transition-transform"
            />
          </div>
          <div class="du-btn du-btn-ghost du-btn-circle size-6 min-h-0 min-w-0">
            <icon-gravity-ui-copy />
          </div>
        </div>
      </div>
      <div v-if="expand" class="flex">
        <Markdown :Content="props.text" ShowTextLoading class="max-w-full min-h-12"/>
        <!-- <p class="text-pretty break-words">
          {{ props.text }}
        </p> -->
      </div>
    </div>
  </div>
</template>
