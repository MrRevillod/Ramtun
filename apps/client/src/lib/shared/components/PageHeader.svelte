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

<div class="mb-4 flex flex-row items-center justify-center pt-2">
	<div class="w-1/3">
		<Title {supra} size="page" {tag}>{title}</Title>
	</div>
	<div class="flex w-2/3 items-center justify-end gap-2">
		{@render children()}
	</div>
</div>
<div class="mb-4 h-px bg-zinc-400/70"></div>
