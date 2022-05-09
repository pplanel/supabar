import preprocess from "svelte-preprocess";
import adapter from "@sveltejs/adapter-auto";

/** @type {import('@sveltejs/kit').Config} */
const config = {
    outDir: "./dist",

  preprocess: [
    preprocess({
      postcss: true,
    }),
  ],
};

export default config;
