<script lang="ts">
	import { QueryClientProvider } from "@tanstack/svelte-query"
	import { browser } from "$app/environment"
	import { Toaster } from "svelte-sonner"
	import "./layout.css"
	import { setupApiInterceptors } from "$lib/shared/http/api.interceptors"
	import { queryClient } from "$lib/shared/query.client"
	import { themeStore } from "$lib/shared/theme/theme.store.svelte"

	let { children } = $props()

	setupApiInterceptors()

	if (browser) {
		themeStore.init()
	}
</script>

<svelte:head>
	<title>INF UCT - Ramtun</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Toaster richColors position="top-right" />
	{@render children()}
</QueryClientProvider>
