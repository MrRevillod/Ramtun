<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { onAttemptsSubmit } from "$lib/shared/socket/attempts.socket"
	import { getErrorMessage } from "$lib/shared/errors"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let { data } = $props()

	const resultsQuery = createQuery(() => ({
		queryKey: ["attempt-result", "managed", data.attemptId],
		queryFn: () => attemptsService.getAttemptResultsManaged(data.attemptId),
	}))

	$effect(() => {
		const unsub = onAttemptsSubmit(payload => {
			if (payload.attemptId === data.attemptId) {
				void resultsQuery.refetch()
			}
		})
		return () => unsub()
	})
</script>

{#if resultsQuery.isLoading}
	<p class="m-0 text-zinc-600">Cargando resultados...</p>
{:else if resultsQuery.error}
	{#if getErrorMessage(resultsQuery.error).includes("no ha sido enviado")}
		<p class="m-0 text-zinc-600">
			El estudiante aún no ha enviado el intento. Los resultados estarán disponibles
			una vez que lo complete.
		</p>
	{:else}
		<p class="m-0 text-red-700">{getErrorMessage(resultsQuery.error)}</p>
	{/if}
{:else if resultsQuery.data}
	<AttemptResultReview result={resultsQuery.data} />
{/if}
