<script lang="ts">
	import type { DateValue } from "$lib/shared/value-objects/date.value"
	import { CircleStop, CircleAlert } from "lucide-svelte"

	interface QuizStatusBadgeProps {
		resultsPublishedAt: DateValue | null
	}

	let { resultsPublishedAt }: QuizStatusBadgeProps = $props()
</script>

{#if resultsPublishedAt}
	<span
		class="inline-flex items-center gap-1 rounded-[4px] bg-zinc-900 px-2.5 py-1 text-xs font-medium text-white"
	>
		<CircleStop size={12} aria-hidden="true" />
		Cerrado
	</span>
{:else}
	<span
		class="inline-flex items-center gap-1 rounded-[4px] border border-zinc-300 px-2.5 py-1 text-xs font-medium text-zinc-600"
	>
		<CircleAlert size={12} aria-hidden="true" />
		Abierto
	</span>
{/if}
