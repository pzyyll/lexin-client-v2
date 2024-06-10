<script setup lang="ts">
useHead({
  htmlAttrs: {
    class: "bg-neutral-bg h-screen",
  },
  bodyAttrs: {
    class: "bg-neutral-bg h-screen",
  },
});

</script>

<template>
    <div class="h-screen">
        <slot />
    </div>
</template>