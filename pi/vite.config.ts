import adapter from "@sveltejs/adapter-static";
import {sveltekit} from "@sveltejs/kit/vite";
import {vitePreprocess} from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import {defineConfig} from "vite";

export default defineConfig({
    plugins: [
        tailwindcss(),
        sveltekit({
            preprocess: vitePreprocess(),
            adapter: adapter({pages: "../assets/pi"}),
            prerender: {
                // Stratagem icons live in the plugin folder (assets/icon), not in the PI, so the
                // prerenderer can't fetch them - they resolve once the plugin is installed.
                handleHttpError: ({path, message}) => {
                    if (path.startsWith("/icon/")) return;
                    throw new Error(message);
                },
            },
        }),
    ],
});