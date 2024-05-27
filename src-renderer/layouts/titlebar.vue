<script setup lang="ts">

</script>

<template>
  <WinTitleBar />
  <div class="container mx-auto">
    <Markdown Content="# TitleBar" />
    <button class="du-btn du-btn-ghost w-10">
      <icon-mdi-window-minimize />
    </button>
    <slot />
    <button class="btn btn-outline">Default</button>
    {{ $t("app_title") }}
    
    
  </div>
</template>
