<script lang="ts">
	import { ArrowRight, Send } from "lucide-svelte"

	interface QuizNavigationProps {
		currentIndex: number
		totalQuestions: number
		isLast: boolean
		isPending: boolean
		autoSubmitting: boolean
		onnavigate: (index: number) => void
		onsubmit: () => void
	}

	let {
		currentIndex,
		totalQuestions,
		isLast,
		isPending,
		autoSubmitting,
		onnavigate,
		onsubmit,
	}: QuizNavigationProps = $props()
</script>

<div class="mt-7 flex flex-wrap justify-end gap-2">
	{#if !isLast}
		<button
			class="btn-secondary flex items-center gap-1.5"
			type="button"
			onclick={() => onnavigate(Math.min(totalQuestions - 1, currentIndex + 1))}
		>
			Siguiente pregunta
			<ArrowRight size={16} aria-hidden="true" />
		</button>
	{:else}
		<button class="btn-primary flex items-center gap-1.5" type="button" onclick={onsubmit}>
			<Send size={16} aria-hidden="true" />
			{isPending || autoSubmitting ? "Enviando..." : "Finalizar intento"}
		</button>
	{/if}
</div>
