import { fromThrowable, type Result, type ResultAsync } from "neverthrow"

import type { AppError } from "$lib/shared/errors"

export { err, ok, ResultAsync } from "neverthrow"
export type { Result } from "neverthrow"

export type AppResult<T> = Result<T, AppError>
export type AppResultAsync<T> = ResultAsync<T, AppError>

export const unwrapResultOrThrow = <T>(result: AppResult<T>): T => {
	const maybe = result as unknown as {
		isErr?: () => boolean
		error?: AppError | null
		value?: T | null
	}

	if (typeof maybe.isErr === "function") {
		if (maybe.isErr()) {
			throw maybe.error
		}

		return maybe.value as T
	}

	if (maybe.error) {
		throw maybe.error
	}

	return maybe.value as T
}
