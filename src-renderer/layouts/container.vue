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
    <div class="container mx-auto h-screen">
        <slot />
    </div>
</template>