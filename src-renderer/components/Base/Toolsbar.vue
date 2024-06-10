<template>
  <div class="flex justify-between items-center content-center">
    <div class="flex items-center">
      <slot name="start"></slot>
    </div>
    <div class="flex items-center">
      <slot name="center"></slot>
    </div>
    <div class="flex items-center">
      <slot name="end"></slot>
    </div>
  </div>
</template>
