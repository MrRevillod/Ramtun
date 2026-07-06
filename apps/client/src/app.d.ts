// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import "axios"
import "@total-typescript/ts-reset"

declare module "axios" {
	interface AxiosRequestConfig<TData = unknown> {
		skipRefresh?: boolean
		_retry?: boolean
		_data?: TData
	}
}

declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {}
