<script lang="ts">
	import { Toaster } from "svelte-sonner"
	import { queryClient } from "$lib/shared/http/tanstack"
	import { QueryClientProvider } from "@tanstack/svelte-query"

	import "./layout.css"

	let { children } = $props()
</script>

<svelte:head>
	<title>INF UCT - Ramtun</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Toaster richColors position="top-right" />
	{@render children()}
</QueryClientProvider>
