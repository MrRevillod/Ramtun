<script lang="ts">
	import type { Snippet } from "svelte"
	import Title from "./Title.svelte"

	let {
		supra,
		title,
		tag,
		children,
	}: {
		supra: string
		title: string
		tag?: "h1" | "h2" | "h3" | "h4"
		children: Snippet
	} = $props()
</script>

<div class="flex flex-row items-center justify-center gap-3">
	<div class="w-1/3">
		<Title {supra} size="page" {tag}>{title}</Title>
	</div>
	<div class="flex w-2/3 items-center justify-end gap-2">
		{@render children()}
	</div>
</div>
<div class="h-px bg-zinc-400/70"></div>
