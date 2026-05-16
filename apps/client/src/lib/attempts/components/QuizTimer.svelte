<script lang="ts">
	import { Clock } from "lucide-svelte"

	interface QuizTimerProps {
		remainingSeconds: number
	}

	let { remainingSeconds }: QuizTimerProps = $props()

	const label = $derived.by(() => {
		const min = Math.floor(remainingSeconds / 60)
		const sec = remainingSeconds % 60
		return `${String(min).padStart(2, "0")}:${String(sec).padStart(2, "0")}`
	})
</script>

<div
	class="notice {remainingSeconds <= 120
		? 'notice-danger'
		: 'notice-warn'} font-medium"
>
	<Clock size={14} class="-mt-0.5 mr-0.5 inline-block" aria-hidden="true" />
	Tiempo restante: {label}
</div>
