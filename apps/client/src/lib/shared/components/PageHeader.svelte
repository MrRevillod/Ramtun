<script lang="ts">
	import type { Snippet } from "svelte"
	import Title from "./Title.svelte"

	interface PageHeaderProps {
		supra: string
		title: string
		tag?: "h1" | "h2" | "h3" | "h4"
		children: Snippet
	}

	let { supra, title, tag, children }: PageHeaderProps = $props()
</script>

<header class="mb-4 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
	<div class="min-w-0">
		<Title {supra} size="page" {tag}>{title}</Title>
	</div>
	<div class="flex flex-wrap items-center gap-2">
		{@render children()}
	</div>
</header>
<div class="keyline mb-4"></div>
