<script setup lang="ts">
import { navigateTo } from "nuxt/app";
import { onMounted } from "vue";
import type { Router } from "vue-router";

const router: Router = useRouter();

watch(router.currentRoute, (to, from) => {
  console.log("to", to);
  console.log("from", from);
});

const isNav = (name) => {
  return router.currentRoute.value.name === name;
};

onMounted(() => {
  navigateTo("/main/chat");
});
</script>

<template>
  <div class="du-drawer md:du-drawer-open">
    <input id="my-drawer-2" type="checkbox" class="du-drawer-toggle" />
    <div class="du-drawer-content flex flex-col">
      <!-- Page content here -->
      <div class="du-navbar bg-base-100">
        <!-- <div class="flex-none">
          <button class="du-btn du-btn-square du-btn-ghost">
          </button>
        </div> -->
        <div class="flex-1">
          <a class="text-2xl">daisyUI</a>
        </div>
      </div>
      <NuxtPage />
    </div>
    <div class="du-drawer-side">
      <!-- <label for="my-drawer-2" aria-label="close sidebar" class="du-drawer-overlay"></label>  -->
      <ul
        class="du-menu p-0 min-h-full border-r bg-neutral text-base-content [&_li>*]:rounded-none"
      >
        <!-- Sidebar content here -->
        <div class="flex-1">
          <li>
            <NuxtLink
              to="/main/chat"
              :class="{ 'du-focus': isNav('main-chat')}"
            >
              <icon-heroicons-chat-bubble-left-right class="size-6" />
            </NuxtLink>
          </li>
        </div>
        <div class="flex-none">
          <li>
            <NuxtLink to="/main/settings" :class="{ 'du-focus': isNav('main-settings') }">
              <icon-fluent-settings-24-regular class="size-6"
            /></NuxtLink>
          </li>
        </div>
      </ul>
    </div>
  </div>
</template>
