<script setup lang="ts">
interface Option {
  name: string;
  code: string;
}

const selected = defineModel<Option>();

const props = defineProps<{
  options?: Option[];
  extra?: string;
}>();
</script>
<template>
  <USelectMenu
    v-model="selected"
    :options="props.options"
    :uiMenu="{
      width: 'w-auto',
      height: 'max-h-28',
      poper: { placement: 'top-start' },
      option: { size: 'text-xs' },
    }"
    :popper="{ placement: 'top-start' }"
    option-attribuite="name"
    searchable
    :search-attributes="['name', 'code']"
    class="unwrapped"
  >
    <button
      class="du-btn du-btn-ghost hover:bg-transparent min-h-0 w-auto max-w-16 size-4 p-0 rounded-none"
    >
      <p class="truncate text-xs">
        {{ selected?.name }}<span v-if="props.extra">:{{ props.extra }}</span>
      </p>
    </button>
    <template #option="{ option }">
      <p>
        {{ option.name }}<span v-if="option.code">({{ option.code }})</span>
      </p>
    </template>
  </USelectMenu>
</template>

<style scoped>
.unwrapped p {
  @apply whitespace-nowrap;
}
</style>
