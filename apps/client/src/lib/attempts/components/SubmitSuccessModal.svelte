<script lang="ts">
	import { fade } from "svelte/transition"
	import { LoaderCircle, CheckCircle2 } from "lucide-svelte"

	interface SubmitSuccessModalProps {
		open: boolean
	}

	let { open }: SubmitSuccessModalProps = $props()
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		transition:fade={{ duration: 150 }}
		role="dialog"
		aria-modal="true"
		tabindex="-1"
	>
		<div class="panel-surface w-full max-w-md p-6" role="presentation" tabindex="-1">
			<div class="flex flex-col items-center gap-4 py-6">
				<CheckCircle2 size={32} class="text-emerald-600" aria-hidden="true" />
				<h3 class="m-0 text-xl text-black">Intento enviado</h3>
				<p class="m-0 text-center text-sm text-zinc-600">
					Tu intento fue enviado correctamente. Los resultados estarán disponibles
					cuando el docente los publique.
				</p>
				<div class="mt-2 flex items-center gap-2 text-sm text-zinc-500">
					<LoaderCircle size={16} class="animate-spin" aria-hidden="true" />
					Redirigiendo...
				</div>
			</div>
		</div>
	</div>
{/if}
