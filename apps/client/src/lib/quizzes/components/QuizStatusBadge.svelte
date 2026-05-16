<script lang="ts">
	import { CircleStop, AlertCircle } from "lucide-svelte"

	interface QuizStatusBadgeProps {
		closedAt: string | null
	}

	let { closedAt }: QuizStatusBadgeProps = $props()
</script>

{#if closedAt}
	<span
		class="inline-flex items-center gap-1 rounded-[4px] bg-zinc-900 px-2 py-1 text-xs text-white"
	>
		<CircleStop size={12} aria-hidden="true" />
		Cerrado
	</span>
{:else}
	<span class="pill-outline">
		<AlertCircle size={12} aria-hidden="true" />
		Abierto
	</span>
{/if}
