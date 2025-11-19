import adapter from "@sveltejs/adapter-static";
import type {Config} from "@sveltejs/kit";
import {vitePreprocess} from "@sveltejs/vite-plugin-svelte";

export default {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({pages: "../assets/pi"}),
        prerender: {
            // Stratagem icons live in the plugin folder (assets/icon), not in te PI, so the
            // prerenderer can't fetch them - they resolve once the plugin is installed.
            handleHttpError: ({path, message}) => {
                if (path.startsWith("/icon/")) return;
                throw new Error(message);
            },
        },
    },
} as Config;
