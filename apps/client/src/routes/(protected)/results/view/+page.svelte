<script lang="ts">
	import type { AttemptResult } from "$lib/attempts/dtos"

	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { onMount } from "svelte"

	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let result = (page.state as { results?: AttemptResult })?.results

	onMount(async () => {
		if (!result) await goto("/results")
	})
</script>

{#if result}
	<AttemptResultReview {result} />
{/if}
