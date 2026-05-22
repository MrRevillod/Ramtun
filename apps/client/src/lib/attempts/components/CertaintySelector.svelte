<script lang="ts">
	import type { CertaintyLevel } from "$lib/attempts/attempts.dtos"
	import { CertaintyValue } from "$lib/shared/value-objects/certainty.value"

	interface CertaintySelectorProps {
		selected: CertaintyLevel | null
		onclick: (level: CertaintyLevel) => void
	}

	let { selected, onclick }: CertaintySelectorProps = $props()
</script>

<p class="mt-7 mb-0 text-sm font-medium text-zinc-800">Nivel de certeza</p>
<div class="mt-2 grid gap-2 sm:grid-cols-3">
	{#each ["low", "medium", "high"] as CertaintyLevel[] as level (level)}
		<button
			type="button"
			class={selected === level ? "btn-primary" : "btn-tertiary"}
			onclick={() => onclick(level)}
		>
			{CertaintyValue.format(level)}
		</button>
	{/each}
</div>
