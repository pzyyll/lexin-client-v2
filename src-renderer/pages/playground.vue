<script setup lang="ts">
import autoAnimate from "@formkit/auto-animate";

const markdown = ref<string>(`
# Hello Worldl
This is a **markdown** text.
and this is a [link](https://www.google.com)
~~~javascript
console.log("Hello World");
~~~
## Title##
### Title
---
- [ ] Task 1
- [x] Task 2
`);

const { x, y } = useMouse();

const dropdown = ref();
const show = ref(false);

onMounted(() => {
  autoAnimate(dropdown.value);
});

onUnmounted(() => {
});
</script>

<template>
  <div>
    <article class="prose">
      <h1>Sphinx of black quartz, judge my vow.</h1>
      <h2>QWERTYUIOPASDFGHJKLZXCVBNM1234567890</h2>
    </article>
    <Alert> This is an auto-imported component. </Alert>
    <icon-mdi-pin-off />
    <icon-mdi-google-translate />
    <icon-svgs-deepl-logo />
    <button class="du-btn"><icon-svgs-deepl-logo /></button>
  </div>
  <UButton loading>Button</UButton>
  <!-- <progress class="progress"></progress> -->
  <div class="flex flex-row">
    <div class="basis-1/4 bg-lime-700">01</div>
    <div class="basis-1/4 bg-stone-400">02</div>
    <div class="basis-1/2 bg-orange-200">03</div>
  </div>
  <button class="du-btn du-btn-square du-btn-outline"></button>

  <div class="du-divider">Markdown</div>
  <UTextarea v-model="markdown" autoresize />
  <Markdown :Content="markdown" />
  <UDivider> ScrollBar </UDivider>
  <div class="h-32 overflow-y-scroll">
    <div class="h-64 bg-slate-400"></div>
  </div>

  <UDivider>Vueuse</UDivider>
  <div>pos: {{ x }}, {{ y }}</div>

  <UDivider>Auto Animate</UDivider>
  <div ref="dropdown" class="dropdown">
    <strong class="dropdown-label" @click="show = !show"> Dropdown </strong>
    <p class="dropdown-content" v-if="show">
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Quisquam, quod.
    </p>
  </div>
</template>

<style scoped></style>
