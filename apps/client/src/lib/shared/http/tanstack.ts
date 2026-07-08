import { QueryClient } from "@tanstack/svelte-query"
import { createQuery as ogCreateQuery } from "@tanstack/svelte-query"
import { createMutation as ogCreateMutation } from "@tanstack/svelte-query"

import type { ApiResponse } from "./response"
import type {
	CreateMutationOptions,
	CreateMutationResult,
	CreateQueryOptions,
	CreateQueryResult,
	QueryKey,
} from "@tanstack/svelte-query"

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			staleTime: 30_000,
			retry: 1,
			refetchOnWindowFocus: false,
		},
		mutations: {
			retry: 0,
		},
	},
})

export function useMutation<TData = unknown, TVariables = void, TContext = unknown>(
	options: () => CreateMutationOptions<
		TData,
		ApiResponse<unknown>,
		TVariables,
		TContext
	>,
): CreateMutationResult<TData, ApiResponse<unknown>, TVariables, TContext> {
	return ogCreateMutation(options as never)
}

export function useQuery<
	TQueryFnData = unknown,
	TData = TQueryFnData,
	TQueryKey extends QueryKey = QueryKey,
>(
	options: () => CreateQueryOptions<
		TQueryFnData,
		ApiResponse<unknown>,
		TData,
		TQueryKey
	>,
): CreateQueryResult<TData, ApiResponse<unknown>> {
	return ogCreateQuery(options as never)
}

export { useQueryClient } from "@tanstack/svelte-query"
