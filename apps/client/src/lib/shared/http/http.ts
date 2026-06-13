import axios, { isAxiosError } from "axios"
import type { AxiosRequestConfig } from "axios"
import type { AppError } from "$lib/shared/errors"

type ApiResponse<T = unknown> = {
	code: number
	success: boolean
	message: string
	timestamp: string
	data?: T | null
	error?: unknown
}

export type ApiRequestConfig<TData = unknown> = AxiosRequestConfig<TData> & {
	skipRefresh?: boolean
	_retry?: boolean
}

export const apiClient = axios.create({
	baseURL: "/api",
	headers: {
		"Content-Type": "application/json",
	},
})

const UNKNOWN_ERROR: AppError = {
	kind: "unknown",
	message: "Ocurrio un error desconocido.",
}

const isApiResponse = (payload: unknown): payload is ApiResponse<unknown> => {
	if (!payload || typeof payload !== "object") {
		return false
	}

	const response = payload as Partial<ApiResponse<unknown>>

	return (
		typeof response.code === "number" &&
		typeof response.success === "boolean" &&
		typeof response.message === "string" &&
		typeof response.timestamp === "string"
	)
}

const mapTransportError = (error: unknown): AppError => {
	if (!isAxiosError(error)) {
		return { ...UNKNOWN_ERROR, details: error }
	}

	if (error.code === "ECONNABORTED") {
		return {
			kind: "network",
			code: "timeout",
			message: "La solicitud tardó demasiado tiempo.",
			details: error,
		}
	}

	if (!error.response) {
		return {
			kind: "network",
			code: "unknown",
			message: "No fue posible conectar con el servidor.",
			details: error,
		}
	}

	const payload = error.response.data

	if (isApiResponse(payload)) {
		return {
			kind: "http",
			status: payload.code,
			message: payload.message,
			details: payload.error ?? null,
		}
	}

	return {
		kind: "http",
		status: error.response.status,
		message: "El servidor respondió con un formato de error no esperado.",
		details: payload,
	}
}

export const request = <T>(config: ApiRequestConfig): Promise<T> =>
	apiClient.request<ApiResponse<T>>(config).then(
		(response): T => {
			if (!isApiResponse(response.data)) {
				throw {
					kind: "decode",
					code: "invalid_envelope",
					message: "Respuesta del servidor inválida.",
					details: response?.data ?? null,
				} satisfies AppError
			}

			const payload = response.data

			if (!payload.success) {
				throw {
					kind: "http",
					status: payload.code,
					message: payload.message,
					details: payload.error ?? null,
				} satisfies AppError
			}

			return payload.data as T
		},
		(error: unknown): never => {
			throw mapTransportError(error)
		},
	)
