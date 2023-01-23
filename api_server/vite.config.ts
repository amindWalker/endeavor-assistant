import presetAttributify from "./unocss-attributify-rust/src/index";
import UnoCSS from "unocss/vite";
import { presetUno } from "@unocss/preset-uno";

export default {
    plugins: [
        UnoCSS({
            presets: [
                presetAttributify({
                    rsxParsing: true,
                }),
                presetUno(),
            ],
            shortcuts: {
                btn: "py-2 px-4 bg-red-400 font-semibold rounded-lg shadow-md",
            },
        }),
    ],
};
