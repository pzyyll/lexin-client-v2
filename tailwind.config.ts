import type { Config } from ".pnpm/tailwindcss@3.4.3/node_modules/tailwindcss";
import daisyui from "daisyui";
import scrollbar from "tailwind-scrollbar";

export default {
  content: ["./src-renderer/**/*.{vue,js,ts,jsx,tsx,html}"],
  theme: {},
  plugins: [daisyui],
  daisyui: {
    prefix: "daisy-", // Change daisyui class prefix
  },
} satisfies Config;
