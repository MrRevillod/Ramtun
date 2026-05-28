<script lang="ts">
	import { resolve } from "$app/paths"
	import { ArrowLeft } from "lucide-svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let { data } = $props()

	const resultsQuery = createQuery(() => ({
		queryKey: ["attempt-result", "managed", data.attemptId],
		queryFn: () => attemptsService.getAttemptResultsManaged(data.attemptId),
	}))
</script>

<section class="panel-elevated p-4 sm:p-6">
	<div class="mb-4 flex items-center justify-between gap-4">
		<h3 class="m-0 text-xl text-black">
			{resultsQuery.data ? `Resultados de ${resultsQuery.data.userName}` : ""}
		</h3>
		<a
			class="btn-tertiary inline-flex shrink-0 items-center gap-1"
			href={resolve(
				`/courses/${data.courseId}/quizzes/${data.quizId}/attempts`
			)}
		>
			<ArrowLeft size={14} />
			Volver a intentos
		</a>
	</div>

	{#if resultsQuery.isLoading}
		<p class="m-0 text-zinc-600">Cargando resultados...</p>
	{:else if resultsQuery.error}
		<p class="m-0 text-red-700">{getErrorMessage(resultsQuery.error)}</p>
	{:else if resultsQuery.data}
		<AttemptResultReview result={resultsQuery.data} />
	{/if}
</section>
