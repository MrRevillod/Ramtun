<script lang="ts">
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import type { AttemptResult } from "$lib/attempts/attempts.dtos"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"
	import { onMount } from "svelte"

	let result = (page.state as { results?: AttemptResult })?.results

	onMount(async () => {
		if (!result) await goto("/results")
	})
</script>

{#if result}
	<AttemptResultReview {result} />
{/if}
