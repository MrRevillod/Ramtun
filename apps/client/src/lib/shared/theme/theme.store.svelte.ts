import { browser } from "$app/environment"

export type ThemePreference = "light" | "dark" | "system"

const STORAGE_KEY = "ui-theme"

const resolveSystemTheme = () => {
	if (!browser) return "light"
	return window.matchMedia("(prefers-color-scheme: dark)").matches
		? "dark"
		: "light"
}

class ThemeStore {
	public preference = $state<ThemePreference>("light")
	public resolved = $state<"light" | "dark">("light")

	private mediaQuery: MediaQueryList | null = null
	private mediaListener: ((event: MediaQueryListEvent) => void) | null = null

	public init() {
		if (!browser) return

		const saved = localStorage.getItem(STORAGE_KEY)
		if (saved === "light" || saved === "dark" || saved === "system") {
			this.preference = saved
		} else {
			this.preference = "light"
		}

		this.mediaQuery = window.matchMedia("(prefers-color-scheme: dark)")
		this.mediaListener = event => {
			if (this.preference === "system") {
				this.resolved = event.matches ? "dark" : "light"
				this.applyThemeClass()
			}
		}

		this.mediaQuery.addEventListener("change", this.mediaListener)
		this.syncResolvedTheme()
	}

	public setPreference(preference: ThemePreference) {
		this.preference = preference
		if (browser) {
			localStorage.setItem(STORAGE_KEY, preference)
		}
		this.syncResolvedTheme()
	}

	public toggle() {
		const next = this.resolved === "dark" ? "light" : "dark"
		this.setPreference(next)
	}

	private syncResolvedTheme() {
		if (this.preference === "system") {
			this.resolved = resolveSystemTheme()
		} else {
			this.resolved = this.preference
		}
		this.applyThemeClass()
	}

	private applyThemeClass() {
		if (!browser) return
		document.documentElement.classList.toggle("dark", this.resolved === "dark")
	}
}

export const themeStore = new ThemeStore()
