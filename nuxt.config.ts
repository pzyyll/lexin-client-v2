import IconsResolver from "unplugin-icons/resolver";
import ViteComponents from "unplugin-vue-components/vite";
import Icons from "unplugin-icons/vite";
import path from "path";
import * as cheerio from "cheerio";

// loader helpers
import { FileSystemIconLoader } from "unplugin-icons/loaders";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  // nitro: {
  //   preset: "static",
  // },
  srcDir: "src-renderer",
  modules: [
    "unplugin-icons/nuxt",
    "@pinia/nuxt",
    "@nuxt/fonts",
    "@nuxt/ui",
    "@vueuse/nuxt",
    "@nuxtjs/i18n",
    "@formkit/auto-animate/nuxt",
    "nuxt-headlessui",
    // "nuxt-primevue",
  ],
  // primevue: {
  //   components: {
  //     prefix: "P",
  //   },
  //   directives: {
  //     prefix: "p-",
  //   },
  //   composables: {
  //     include: ["*", { name: "usePrimeVueToast", use: { as: "useToast" } }],
  //   },
  //   options: {
  //     unstyled: true,
  //   },
  // },
  i18n: {
    vueI18n: "./i18n.config.ts",
  },
  headlessui: {
    prefix: "H",
  },
  vite: {
    build: {
      ssr: false,
      chunkSizeWarningLimit: 1024,
      // rollupOptions: {
      //   output: {
      //     manualChunks(id) {
      //       if (id.includes("node_modules/")) {
      //         const splits = id.toString().split("node_modules/").at(-1)?.split("/");
      //         console.log(`Chunking ${splits}`);
      //         const name = splits?.at(0);
      //         console.log(`Chunking ${name}`);
      //         return name;
      //       }
      //     },
      //   },
      // },
    },
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://tauri.app/2/reference/environment-variables/
    // envPrefix: ["VITE_", "TAURI_"],
    server: {
      watch: {
        ignored: ["**/src-tauri/**"],
      },
    },
    plugins: [
      ViteComponents({
        resolvers: [
          IconsResolver({
            prefix: "icon",
            customCollections: ["svgs"],
          }),
        ],
      }),
      Icons({
        autoInstall: true,
        compiler: "vue3",
        customCollections: {
          svgs: FileSystemIconLoader(
            path.resolve(__dirname, "src-renderer/assets/icons"),
            (svg) => {
              // Replace fill attribute with currentColor if exists, sets it if it doesn't.
              // and delete width and height attributes if they exist.
              const $ = cheerio.load(svg, { xmlMode: true });
              const $svg = $("svg");
              $svg.attr("fill", "currentColor");
              $svg.removeAttr("width");
              $svg.removeAttr("height");
              return $.xml($svg);
            }
          ),
        },
        iconCustomizer(collection, icon, props) {
          if (collection === "svgs") {
            props.width = "1.5em";
            props.height = "1.5em";
          }
        },
      }),
    ],
  },
  devServer: {
    port: 1234,
  },
  $development: {
    devtools: {
      enabled: true,
    },
  },
});
