const DEFAULT_ERROR_MESSAGE =
	"Ocurrió un error inesperado, por favor intente nuevamente más tarde."

export class ApiResponse<T = unknown> {
	constructor(
		public code: number,
		public success: boolean,
		public message: string,
		public timestamp: string,
		public data?: T | null,
		public error?: unknown,
		public errors?: ValidationErrors | null
	) {}

	static is(payload: unknown): payload is ApiResponse<unknown> {
		if (!payload || typeof payload !== "object") return false

		const r = payload as Record<string, unknown>

		return (
			typeof r.code === "number" &&
			typeof r.success === "boolean" &&
			typeof r.message === "string" &&
			typeof r.timestamp === "string"
		)
	}

	static genericError(code: number, message: string): ApiResponse<never> {
		return new ApiResponse<never>(code, false, message, new Date().toISOString())
	}

	static from(payload: ApiResponse<unknown>): ApiResponse<unknown> {
		return new ApiResponse<unknown>(
			payload.code,
			payload.success,
			payload.message,
			payload.timestamp,
			payload.data,
			payload.error,
			payload.errors
		)
	}

	get messageOrDefault(): string {
		return this.message ?? DEFAULT_ERROR_MESSAGE
	}

	static messageOrDefault(error: unknown): string {
		if (ApiResponse.is(error)) return error.messageOrDefault
		if (error instanceof Error) return error.message
		return DEFAULT_ERROR_MESSAGE
	}
}

export type ValidationErrors = Record<string, { code: string; message: string }[]>
