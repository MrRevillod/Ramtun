<script lang="ts">
	import Markdown from "$lib/shared/components/Markdown.svelte"

	let { value = $bindable("") }: { value?: string } = $props()

	let mode = $state<"write" | "preview">("write")
</script>

<div class="grid gap-2">
	<div class="flex items-center gap-1">
		<button
			type="button"
			class={mode === "write" ? "btn-secondary" : "btn-tertiary"}
			onclick={() => (mode = "write")}
		>
			Escribir
		</button>
		<button
			type="button"
			class={mode === "preview" ? "btn-secondary" : "btn-tertiary"}
			onclick={() => (mode = "preview")}
		>
			Vista previa
		</button>
	</div>

	{#if mode === "write"}
		<textarea
			class="input-base min-h-32 w-full resize-y font-mono"
			bind:value
			placeholder="Escribe el enunciado en Markdown..."
		></textarea>
	{:else}
		<div class="input-base min-h-32 overflow-y-auto">
			{#if value}
				<Markdown content={value} />
			{:else}
				<p class="m-0 text-sm text-zinc-500">Sin contenido para previsualizar.</p>
			{/if}
		</div>
	{/if}
</div>
