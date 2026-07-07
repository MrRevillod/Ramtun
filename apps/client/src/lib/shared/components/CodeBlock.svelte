<script lang="ts">
	import { Copy, Check } from "lucide-svelte"
	import { inlineTryAsync } from "$lib/shared/try"

	interface CodeBlockProps {
		code: string
		class?: string
	}

	let { code, class: className = "" }: CodeBlockProps = $props()

	let copied = $state(false)

	const copyToClipboard = () => {
		inlineTryAsync(async () => {
			await navigator.clipboard.writeText(code)
			copied = true
			setTimeout(() => {
				copied = false
			}, 2000)
		})
	}
</script>

<div class="relative {className}">
	<pre class="overflow-x-auto rounded-md bg-zinc-50 p-4 text-sm text-zinc-800">{code}</pre>

	<button
		type="button"
		class="absolute top-2 right-2 rounded p-1 text-zinc-400 transition-colors hover:bg-zinc-200 hover:text-zinc-600"
		onclick={copyToClipboard}
		aria-label="Copiar código"
	>
		{#if copied}
			<Check size={14} class="text-emerald-600" />
		{:else}
			<Copy size={14} />
		{/if}
	</button>
</div>
