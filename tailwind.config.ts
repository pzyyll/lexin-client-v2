import type { Config } from ".pnpm/tailwindcss@3.4.3/node_modules/tailwindcss";
import daisyui from "daisyui";

export default {
  content: ["./src-renderer/**/*.{vue,js,ts,jsx,tsx,html}"],
  theme: {
    extend: {
      colors: {
        primary: "#222831",
        secondary: "#31363F",
        cool: "#76ABAE",
        neutral: "#EEEEEE"
      }
    }
  },
  plugins: [daisyui],
  daisyui: {
    prefix: "du-", // Change daisyui class prefix
  },
} satisfies Config;
