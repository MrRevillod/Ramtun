import { browser } from "$app/environment"

export type ThemePreference = "light" | "dark"

const STORAGE_KEY = "ui-theme"

class ThemeStore {
	public preference = $state<ThemePreference>("light")

	public init() {
		if (!browser) return

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
