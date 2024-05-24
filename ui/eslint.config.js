import globals from "globals";
import pluginJs from "@eslint/js";
import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";

export default [
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ...globals.node,
            },
        },
    },
    pluginJs.configs.recommended,
    eslintPluginPrettierRecommended,
    {
        rules: {
            "prettier/prettier": [
                "error",
                {
                    singleQuote: false,
                    semi: true,
                    trailingComma: "all",
                    tabWidth: 4,
                    endOfLine: "lf",
                },
            ],
        },
    },
    {
        ignores: ["dist/"],
    },
];
