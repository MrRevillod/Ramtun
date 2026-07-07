<script lang="ts">
	import { Toaster } from "svelte-sonner"
	import { queryClient } from "$lib/shared/http/query-client"
	import { QueryClientProvider } from "@tanstack/svelte-query"
	import { onMount } from "svelte"

	import "./layout.css"

	let { children } = $props()

	onMount(() => {
		localStorage.removeItem("ui-theme")
	})
</script>

<svelte:head>
	<title>INF UCT - Ramtun</title>
</svelte:head>

<QueryClientProvider client={queryClient}>
	<Toaster richColors position="top-right" />
	{@render children()}
</QueryClientProvider>
