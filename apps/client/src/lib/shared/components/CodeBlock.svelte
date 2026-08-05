<script lang="ts">
	import { Copy, Check } from "lucide-svelte"
	import { inlineTryAsync } from "$lib/shared/try"

	interface CodeBlockProps {
		code: string
		class?: string
	}

	let { code, class: className = "" }: CodeBlockProps = $props()

	let copied = $state(false)

	const copyToClipboard = async () => {
		await inlineTryAsync(async () => {
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
		<span class="relative block">
			<span
				class="absolute inset-0 flex items-center justify-center transition-[opacity,scale,filter] duration-300 ease-[cubic-bezier(0.2,0,0,1)] {copied
					? 'blur-0 scale-100 opacity-100'
					: 'scale-[0.25] opacity-0 blur-[4px]'}"
			>
				<Check size={14} class="text-emerald-600" />
			</span>
			<span
				class="block transition-[opacity,scale,filter] duration-300 ease-[cubic-bezier(0.2,0,0,1)] {copied
					? 'scale-[0.25] opacity-0 blur-[4px]'
					: 'blur-0 scale-100 opacity-100'}"
			>
				<Copy size={14} />
			</span>
		</span>
	</button>
</div>
