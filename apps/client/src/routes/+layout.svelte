<script lang="ts">
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"
	import { Toaster } from "svelte-sonner"
	import { setupApiInterceptors } from "$lib/shared/http/api.interceptors"
	import { themeStore } from "$lib/shared/theme.store.svelte"

	import "./layout.css"

	let { children } = $props()

	setupApiInterceptors()

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				staleTime: 30_000,
				refetchOnWindowFocus: false,
				retry: false,
			},
			mutations: {
				retry: false,
			},
		},
	})

	themeStore.init()
</script>

<svelte:head>
	<title>INF UCT - Ramtun</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Toaster richColors position="top-right" />
	{@render children()}
</QueryClientProvider>
