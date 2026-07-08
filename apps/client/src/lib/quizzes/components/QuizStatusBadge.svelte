<script lang="ts">
	import type { DateValue } from "$lib/shared/value-objects/date.value"
	import { CircleStop, CircleAlert } from "lucide-svelte"

	interface QuizStatusBadgeProps {
		resultsPublishedAt: DateValue | null
	}

	let { resultsPublishedAt }: QuizStatusBadgeProps = $props()
</script>

{#if resultsPublishedAt}
	<span class="inline-flex items-center gap-1 rounded-sm bg-zinc-900 px-2 py-1 text-xs text-white">
		<CircleStop size={12} aria-hidden="true" />
		Cerrado
	</span>
{:else}
	<span class="pill-outline">
		<CircleAlert size={12} aria-hidden="true" />
		Abierto
	</span>
{/if}
