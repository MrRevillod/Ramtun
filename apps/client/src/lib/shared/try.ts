type Result<T, E = Error> = [T | null, E | null]

export const inlineTry = <T, E = Error>(fn: () => T): Result<T, E> => {
	try {
		return [fn(), null]
	} catch (error) {
		return [null, error as E]
	}
}

export const inlineTryAsync = async <T, E = Error>(fn: () => Promise<T>): Promise<Result<T, E>> => {
	try {
		return [await fn(), null]
	} catch (error) {
		return [null, error as E]
	}
}
