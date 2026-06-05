import type { WarningType } from "$lib/attempts/attempts.dtos"
import { attemptsService } from "$lib/attempts/attempts.service"

type WarningHandler = (type: WarningType, details: string) => void

const DEBOUNCE_MS = 2000

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
	const preventContextMenu = (e: MouseEvent) => {
		e.preventDefault()
		fire(
			attemptId,
			"clipboard",
			"No está permitido abrir el menú contextual en un cuestionario.",
			onWarning
		)
	}

	const onKeyDown = (e: KeyboardEvent) => {
		if ((e.ctrlKey || e.metaKey) && e.key === "c") {
			fire(
				attemptId,
				"clipboard",
				"No está permitido copiar texto en un cuestionario.",
				onWarning
			)
			return
		}

		if (e.key === "PrintScreen") {
			fire(
				attemptId,
				"screenshot",
				"No está permitido capturar pantalla en un cuestionario.",
				onWarning
			)
			return
		}

		if (e.key === "Alt" || e.key === "Meta") {
			fire(
				attemptId,
				"navigation",
				"No está permitido usar teclas de navegación del sistema en un cuestionario.",
				onWarning
			)
		}
	}

	let focusLossFired = false
	const onBlur = () => {
		focusLossFired = true
		fire(
			attemptId,
			"focus_loss",
			"No está permitido salir de la ventana en un cuestionario.",
			onWarning
		)
	}

	const onVisibilityChange = () => {
		if (document.hidden && !focusLossFired) {
			fire(
				attemptId,
				"focus_loss",
				"No está permitido salir de la ventana en un cuestionario.",
				onWarning
			)
		}
		if (!document.hidden) {
			focusLossFired = false
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
		document.addEventListener("visibilitychange", onVisibilityChange)
		devtoolsCheckId = setInterval(checkDevTools, 3000)
	}

	const stop = () => {
		document.removeEventListener("contextmenu", preventContextMenu)
		window.removeEventListener("keydown", onKeyDown)
		window.removeEventListener("blur", onBlur)
		document.removeEventListener("visibilitychange", onVisibilityChange)
		if (devtoolsCheckId) {
			clearInterval(devtoolsCheckId)
			devtoolsCheckId = null
		}
	}

	return { start, stop }
}
