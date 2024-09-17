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

const takeScreenshot = async () => {
  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d");
  const stream = await navigator.mediaDevices.getDisplayMedia({
    video: true,
  });
  const video = document.createElement("video");
  video.srcObject = stream;
  video.play();
  video.addEventListener("loadeddata", () => {
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    ctx?.drawImage(video, 0, 0);
  });
};

const items = ref([
  { name: "Amy Elsner", image: "amyelsner.png" },
  { name: "Anna Fali", image: "annafali.png" },
  { name: "Asiya Javayant", image: "asiyajavayant.png" },
]);

onMounted(() => {
  autoAnimate(dropdown.value);
});

onUnmounted(() => {});
</script>

<template>
  <div class="container mx-auto mb-4">
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
    <progress class="du-progress rounded-none"></progress>
    <UDivider>Screenshot</UDivider>
    <button @click="takeScreenshot">Take Screenshot</button>
    <canvas id="canvas" height="100" class="bg-slate-200 w-full"></canvas>

    <UDivider>Radio</UDivider>
    <div class="du-join">
      <input
        class="du-join-item du-btn"
        type="radio"
        name="options"
        aria-label="Radio 1"
      />
      <input
        class="du-join-item du-btn"
        type="radio"
        name="options"
        aria-label="Radio 2"
      />
      <input
        class="du-join-item du-btn"
        type="radio"
        name="options"
        aria-label="Radio 3"
      />
    </div>

    <div class="du-card w-96 bg-base-100 shadow-xl p-2">
      <div class="du-card-body">
        <h2 class="du-card-title">Shoes!</h2>
        <p>If a dog chews shoes whose shoes does he choose?</p>
        <div class="du-card-actions justify-end">
          <button class="du-btn du-btn-primary">Buy Now</button>
        </div>
      </div>
    </div>

    <textarea class="du-textarea du-textarea-primary" placeholder="Bio"></textarea>
    <div>
      <UPopover :popper="{ placement: 'top-start' }">
        <!-- <UButton
        color="white"
        label="Open"
        trailing-icon="i-heroicons-chevron-down-20-solid"
      /> -->
        <input class="du-input" type="text" />

        <template #panel>
          <div class="p-4">
            <div class="h-20 w-48 bg-slate-200"></div>
          </div>
        </template>
      </UPopover>
      <input class="du-input" type="text" />
    </div>
  </div>
  <div>
    <p class="p-after-loading">efg</p>
    <h1 class="p-after-loading">abc</h1>
  </div>
  <!-- <PButton label="Submit" class="du-btn" /> -->
  <div class="du-divider">BaseToolsbar</div>
  <BaseToolsbar>
    <template #center>
      <icon-clarity-pin-line />
    </template>
    <template #end>
      <SvgsIcon name="IconDeepL" />
    </template>
  </BaseToolsbar>
</template>

<style scoped>
.p-after-loading::after {
  content: "";
  display: block;
  position: relative;
  pointer-events: none;
  top: 0.45em;
  @apply du-loading du-loading-dots du-loading-xs z-0;
}
</style>
