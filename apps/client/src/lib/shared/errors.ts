export type AppError =
	| {
		kind: "auth"
		code: "missing_refresh_token" | "session_expired" | "invalid_session"
		message: string
		details?: unknown
	  }
	| {
		kind: "http"
		status: number
		message: string
		details?: unknown
	  }
	| {
		kind: "network"
		code: "offline" | "timeout" | "aborted" | "unknown"
		message: string
		details?: unknown
	  }
	| {
		kind: "decode"
		code: "invalid_envelope" | "invalid_payload"
		message: string
		details?: unknown
	  }
	| {
		kind: "unknown"
		message: string
		details?: unknown
	  }

const isAppError = (error: unknown): error is AppError => {
	if (!error || typeof error !== "object") return false

	return "kind" in error && "message" in error
}

export const getErrorMessage = (error: unknown): string => {
	if (isAppError(error)) {
		return error.message
	}

	if (error instanceof Error && error.message) {
		return error.message
	}

	return "Ocurrio un error inesperado."
}
