<script lang="ts">
	import type { Snippet } from "svelte"

	interface TitleProps {
		supra?: string
		size?: "page" | "subpage"
		tag?: "h1" | "h2" | "h3" | "h4"
		children: Snippet
	}

	let { supra, size = "subpage", tag = "h2", children }: TitleProps = $props()
</script>

{#if supra}
	<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">{supra}</p>
{/if}
<svelte:element
	this={tag}
	class="mt-0.5 mb-0 tracking-tight text-black {size === 'page' ? 'text-[22px]' : 'text-2xl'}"
>
	{@render children()}
</svelte:element>
