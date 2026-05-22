import { browser } from "$app/environment"

const STORAGE_KEY = "ui-theme"

export type ThemePreference = "light" | "dark"

class ThemeStore {
	public preference = $state<ThemePreference>("light")

	public init() {
		const saved = localStorage.getItem(STORAGE_KEY)

		if (!saved || (saved !== "light" && saved !== "dark")) {
			this.preference = "light"
		}

		this.preference = saved! as ThemePreference

		this.applyThemeClass()
	}

	public setPreference(preference: ThemePreference) {
		this.preference = preference

		if (browser) {
			localStorage.setItem(STORAGE_KEY, preference)
		}

		this.applyThemeClass()
	}

	public toggle() {
		this.setPreference(this.preference === "dark" ? "light" : "dark")
	}

	private applyThemeClass() {
		if (!browser) return
		document.documentElement.classList.toggle("dark", this.preference === "dark")
	}
}

export const themeStore = new ThemeStore()
