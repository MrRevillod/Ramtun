import type { WarningType } from "$lib/attempts/attempts.dtos"
import { attemptsService } from "$lib/attempts/attempts.service"

type WarningHandler = (type: WarningType, details: string) => void

const DEBOUNCE_MS = 1000

const lastEmit: Record<string, number> = {}

const canEmit = (key: string) => {
	const now = Date.now()
	if (lastEmit[key] && now - lastEmit[key] < DEBOUNCE_MS) return false
	lastEmit[key] = now
	return true
}

const fire = (
	attemptId: string,
	type: WarningType,
	details: string,
	onWarning?: WarningHandler
) => {
	if (!canEmit(type)) return
	void attemptsService.recordWarning(attemptId, type, details)
	onWarning?.(type, details)
}

export const createAntiCheat = (attemptId: string, onWarning?: WarningHandler) => {
	let focusLossFired = false

	const preventContextMenu = (e: MouseEvent) => {
		e.preventDefault()
		fire(
			attemptId,
			"context_menu",
			"No está permitido abrir el menú contextual en un cuestionario.",
			onWarning
		)
	}

	const onKeyDown = (e: KeyboardEvent) => {
		if ((e.ctrlKey || e.metaKey) && e.key === "c") {
			e.preventDefault()
			fire(
				attemptId,
				"copy_attempt",
				"No está permitido copiar texto en un cuestionario.",
				onWarning
			)
			return
		}

		if (e.key === "PrintScreen") {
			e.preventDefault()
			fire(
				attemptId,
				"screenshot",
				"No está permitido capturar pantalla en un cuestionario.",
				onWarning
			)
			return
		}

		if ((e.ctrlKey || e.metaKey) && (e.key === "f" || e.key === "F")) {
			e.preventDefault()
			fire(
				attemptId,
				"search_attempt",
				"No está permitido buscar en la página durante un cuestionario.",
				onWarning
			)
			return
		}
	}

	const onBlur = () => {
		if (focusLossFired) return
		focusLossFired = true
		fire(
			attemptId,
			"window_blur",
			"La ventana del cuestionario perdió el foco.",
			onWarning
		)
	}

	const onFocus = () => {
		setTimeout(() => {
			focusLossFired = false
		}, 100)
	}

	const onVisibilityChange = () => {
		if (document.hidden && !focusLossFired) {
			focusLossFired = true
			fire(
				attemptId,
				"tab_hide",
				"Se cambió de pestaña durante el cuestionario.",
				onWarning
			)
		}
	}

	const DEVTOOLS_THRESHOLD = 160
	let devtoolsCheckId: ReturnType<typeof setInterval> | null = null

	const checkDevTools = () => {
		const widthDiff = window.outerWidth - window.innerWidth
		const heightDiff = window.outerHeight - window.innerHeight

		if (widthDiff > DEVTOOLS_THRESHOLD || heightDiff > DEVTOOLS_THRESHOLD) {
			fire(
				attemptId,
				"devtools",
				"No está permitido abrir herramientas de desarrollador en un cuestionario.",
				onWarning
			)
		}
	}

	const start = () => {
		document.addEventListener("contextmenu", preventContextMenu)
		window.addEventListener("keydown", onKeyDown)
		window.addEventListener("blur", onBlur)
		window.addEventListener("focus", onFocus)
		document.addEventListener("visibilitychange", onVisibilityChange)
		devtoolsCheckId = setInterval(checkDevTools, 3000)
	}

	const stop = () => {
		document.removeEventListener("contextmenu", preventContextMenu)
		window.removeEventListener("keydown", onKeyDown)
		window.removeEventListener("blur", onBlur)
		window.removeEventListener("focus", onFocus)
		document.removeEventListener("visibilitychange", onVisibilityChange)
		if (devtoolsCheckId) {
			clearInterval(devtoolsCheckId)
			devtoolsCheckId = null
		}
	}

	return { start, stop }
}
