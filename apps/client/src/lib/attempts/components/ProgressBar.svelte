<script lang="ts">
	interface ProgressBarProps {
		current: number
		total: number
	}

	let { current, total }: ProgressBarProps = $props()

	const percent = $derived(total > 0 ? (current / total) * 100 : 0)
</script>

<div class="h-1.5 w-56 overflow-hidden rounded-full bg-zinc-200">
	<div
		class="h-full rounded-full bg-zinc-900 transition-all duration-200"
		style={`width: ${percent}%`}
	></div>
</div>
