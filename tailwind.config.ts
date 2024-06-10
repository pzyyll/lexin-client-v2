import type { Config } from ".pnpm/tailwindcss@3.4.3/node_modules/tailwindcss";
import daisyui from "daisyui";
import htailwind from "@headlessui/tailwindcss";

export default {
  content: ["./src-renderer/**/*.{vue,js,ts,jsx,tsx,html}"],
  theme: {
    extend: {
      colors: {
        primary0: "#222831",
        secondary0: "#31363F",
        cool: "#76ABAE",
        neutral: "#EEEEEE",
        "neutral-bg": "#E8E8E8",
      },
    },
  },
  plugins: [daisyui, htailwind],
  daisyui: {
    prefix: "du-", // Change daisyui class prefix
  },
} satisfies Config;
