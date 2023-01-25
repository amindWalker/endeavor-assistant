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
            scale: 1.5,
            extraProperties: {
                display: "inline-block",
            },
        }),
        presetTypography(),
        presetWebFonts({
            provider: "google",
            fonts: {
                // sans: "Roboto",
                // mono: ["Fira Code", "Fira Mono:400,700"],
                // // custom
                // lobster: "Lobster",
                // lato: [
                //     {
                //         name: "Lato",
                //         weights: ["400", "700"],
                //         italic: true,
                //     },
                //     {
                //         name: "sans-serif",
                //         provider: "none",
                //     },
                // ],
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
        base: "@apply bg-gradient-to-t from-slate-200 to-slate-50 rounded-lg p4 drop-shadow",
        btn: "m1 text-white filter-none bg-gradient-to-tr from-orange-600 to-orange-500 hover:(filter brightness-130 drop-shadow-xl) -sm:(p4 text-xl rounded-lg) p3 text-lg rounded-md filter drop-shadow-md",
        "btn-2": "m1 text-orange-600 border-b-orange-600 filter-none focus:ring hover:(filter brightness-130 drop-shadow-xl) -sm:(p4 text-xl rounded-lg) p3 text-lg textsha rounded-md filter drop-shadow-md"
    },
});
