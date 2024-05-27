<script setup lang="ts">
definePageMeta({
  layout: 'titlebar'
})

useHead({
    htmlAttrs: {
        class: 'bg-transparent'
    },
    bodyAttrs: {
        class: 'bg-transparent'
    }
})

</script>

<template>
    <div class="flex">
        <div class="flex-1">
            01
        </div>
        <div class="du-divider"></div> <!-- Add this line -->
        <div class="flex-1">
            02
        </div>
    </div>
</template>