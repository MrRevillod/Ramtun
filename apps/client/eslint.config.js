import ts from "typescript-eslint"
import js from "@eslint/js"
import path from "node:path"
import svelte from "eslint-plugin-svelte"
import globals from "globals"
import prettier from "eslint-config-prettier"

import { defineConfig } from "eslint/config"
import { includeIgnoreFile } from "@eslint/compat"

const gitignorePath = path.resolve(import.meta.dirname, "../../.gitignore")

export default defineConfig(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	ts.configs.recommended,
	svelte.configs.recommended,
	prettier,
	svelte.configs.prettier,
	{
		languageOptions: { globals: { ...globals.browser, ...globals.node } },
		rules: {
			"no-undef": "off",
			"svelte/no-navigation-without-resolve": "off",
			"@typescript-eslint/no-explicit-any": "off",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
				},
			],
			"@typescript-eslint/consistent-type-imports": "error",
		},
	},
	{
		files: ["**/*.ts", "**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: [".svelte"],
				parser: ts.parser,
			},
		},
		rules: {
			// "@typescript-eslint/no-floating-promises": "error",
			"@typescript-eslint/no-misused-promises": "error",
			"@typescript-eslint/restrict-template-expressions": "error",
			"@typescript-eslint/restrict-plus-operands": "error",
			"@typescript-eslint/switch-exhaustiveness-check": "error",
			"@typescript-eslint/prefer-readonly": "error",
			"@typescript-eslint/no-unnecessary-type-assertion": "error",
		},
	},
	{
		rules: {},
	}
)
