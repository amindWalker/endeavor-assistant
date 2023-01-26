import { defineConfig } from "unocss/vite";
import {
    presetIcons,
    presetTypography,
    presetWebFonts,
    presetUno,
} from "unocss";
import type { Theme } from "unocss/preset-uno";
import presetTheme from "unocss-preset-theme";

export default defineConfig({
    presets: [
        presetIcons({
            scale: 1.25,
            extraProperties: {
                display: "inline-block",
            },
        }),
        presetTypography(),
        presetWebFonts({
            provider: "google",
            fonts: {
            },
        }),
        presetUno<Theme>({}),
        presetTheme<Theme>({
            theme: {
                base: {
                    colors: {
                    },
                },
                // dark themes
                dark: {
                    colors: {
                    },
                },
            },
        }),
    ],
    // TODO: create base, light and dark themes and reuseable components
    shortcuts: {
        "base-container": "@apply bg-gradient-to-t from-zinc-200 to-zinc-50 rounded-lg p4 drop-shadow",
        "main-std": "all:transition-all all:duration-500 grid grid-cols-6 gap8 mt10 max-w-full min-h-100vh bg-gray-100",
        "btn-primary": "text-white filter-none bg-gradient-to-tr from-orange-600 to-orange-400 hover:(filter brightness-130 drop-shadow-xl) filter drop-shadow-lg rounded-md p3",
        "btn-transparent": "text-orange-600 filter-none ring-inset hover:(filter brightness-130 drop-shadow-xl) filter drop-shadow-lg rounded-md p3 active:(bg-white ring ring-orange-600) focus:(bg-white ring ring-orange-600)",
        "btn-secondary": "z10 text-orange-600 filter-none ring ring-inset ring-orange-600 hover:(filter brightness-130 drop-shadow-xl) filter drop-shadow-lg rounded-md p3 active:(bg-white ring ring-orange-600) focus:(bg-white ring ring-orange-600)",
        "task-list-item": "text-blue-900 shadow p4 ring rounded-xl hover:(bg-white bg-opacity-80)",
        "task-block": "grid gap2 p4 rounded-xl shadow-inner",
        "top-notification": "backdrop-filter backdrop-blur fixed z10 bg-white bg-opacity-50 w-full lg:col-end-6 text-center p2 text-red drop-shadow-lg -mt10",
        "h-title": "text-lg font-bold text-gray-500",
        "p-description": "text-gray-500",
        "text-field": "filter-none hover:(filter drop-shadow ring ring-orange-600) focus:ring row-span-1 -sm:(text-lg p-4 rounded-lg) p-3 rounded-md resize",
        "text-area": ""
    },
});
