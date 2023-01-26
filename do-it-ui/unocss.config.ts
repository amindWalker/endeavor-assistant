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
        base: "@apply bg-gradient-to-t from-slate-200 to-slate-50 rounded-lg p4 drop-shadow",
        "btn-primary": "text-white filter-none bg-gradient-to-tr from-orange-600 to-orange-400 hover:(filter brightness-130 drop-shadow-xl) filter drop-shadow-lg rounded-md m1 p3 text-md md:(text-xl)",
        "btn-transparent": "text-orange-600 filter-none ring-inset hover:(filter brightness-130 drop-shadow-xl) filter drop-shadow-lg rounded-md p3 text-md md:(text-xl) active:(bg-white ring ring-orange-600) focus:(bg-white ring ring-orange-600)"
    },
});
