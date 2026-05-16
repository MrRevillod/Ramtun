<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { fade } from "svelte/transition"
	import { X, CheckCircle2, ArrowUpRight } from "lucide-svelte"

	interface SubmitSuccessModalProps {
		open: boolean
		joinCode: string
		onautoclose: () => void
	}

	let { open, joinCode, onautoclose }: SubmitSuccessModalProps = $props()
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		transition:fade={{ duration: 150 }}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={onautoclose}
		onkeydown={e => {
			if (e.key === "Escape") onautoclose()
		}}
	>
		<div
			class="panel-surface w-full max-w-md p-6"
			role="presentation"
			tabindex="-1"
			onclick={e => e.stopPropagation()}
		>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="m-0 flex items-center gap-2 text-xl text-black">
					<CheckCircle2 size={20} class="text-emerald-600" aria-hidden="true" />
					Intento enviado
				</h3>
				<button class="btn-tertiary p-1" type="button" onclick={onautoclose}>
					<X size={18} aria-hidden="true" />
				</button>
			</div>
			<p class="mb-4 text-sm text-zinc-700">
				Tu intento fue enviado correctamente. Los resultados estarán disponibles
				cuando el docente los publique.
			</p>
			<div class="grid gap-2">
				<button
					class="btn-primary flex items-center justify-center gap-1.5"
					type="button"
					onclick={() =>
						goto(resolve(`/results/lobby?joinCode=${encodeURIComponent(joinCode)}`))}
				>
					<ArrowUpRight size={16} aria-hidden="true" />
					Ir a sala de espera de resultados
				</button>
				<button
					class="btn-secondary"
					type="button"
					onclick={() => goto(resolve("/"))}>Volver al Inicio</button
				>
			</div>
		</div>
	</div>
{/if}
