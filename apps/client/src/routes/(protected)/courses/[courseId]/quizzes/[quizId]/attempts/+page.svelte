<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { createQuery } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import {
		onAttemptNew,
		onAttemptsSubmit,
		onAttemptWarning,
	} from "$lib/shared/socket/attempts.socket"
	import { getErrorMessage } from "$lib/shared/errors"
	import { GradeValue } from "$lib/shared/value-objects/grade.value"
	import { DateValue } from "$lib/shared/value-objects/date.value"

	let { data } = $props()

	const attemptsQuery = createQuery(() => ({
		queryKey: ["attempts", "managed", data.courseId, data.quizId],
		queryFn: () => attemptsService.listAttempts(data.courseId, data.quizId),
	}))

	const viewAttempt = (attemptId: string) => {
		void goto(
			resolve(
				`/courses/${data.courseId}/quizzes/${data.quizId}/attempts/${attemptId}`
			)
		)
	}

	const warningBadge = (count: number) => {
		if (count === 0) return "text-zinc-400"
		if (count <= 2) return "bg-amber-100 text-amber-800"
		if (count <= 5) return "bg-orange-100 text-orange-800"
		return "bg-red-100 text-red-800"
	}

	$effect(() => {
		const unsubSubmit = onAttemptsSubmit(payload => {
			if (payload.quizId !== data.quizId) return
			void attemptsQuery.refetch()
		})

		const unsubNew = onAttemptNew(() => {
			void attemptsQuery.refetch()
		})

		const unsubWarning = onAttemptWarning(() => {
			void attemptsQuery.refetch()
		})

		return () => {
			unsubSubmit()
			unsubNew()
			unsubWarning()
		}
	})
</script>

<section class="grid gap-4">
	<header>
		<h3 class="mt-2 mb-0 text-xl text-black">{data.quizName} - Intentos</h3>
		<p class="m-0 mt-2 text-zinc-700">
			Revisa los intentos realizados por los estudiantes en este cuestionario.
		</p>
	</header>

	<section>
		{#if attemptsQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando intentos...</p>
		{:else if attemptsQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(attemptsQuery.error)}</p>
		{:else if !attemptsQuery.data?.length}
			<p class="notice notice-warn m-0">
				Aún no hay intentos registrados para este cuestionario.
			</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre del estudiante</th>
							<th class="px-3 py-2 text-left font-medium">Hora de inicio</th>
							<th class="px-3 py-2 text-left font-medium">Hora de envío</th>
							<th class="px-3 py-2 text-left font-medium">Puntos</th>
							<th class="px-3 py-2 text-left font-medium">Nota</th>
							<th class="px-3 py-2 text-left font-medium">Alertas</th>
						</tr>
					</thead>
					<tbody>
						{#each attemptsQuery.data as attempt (attempt.attemptId)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() => viewAttempt(attempt.attemptId)}
							>
								<td class="px-3 py-2 font-medium text-zinc-900"
									>{attempt.userName}</td
								>
								<td class="px-3 py-2 text-zinc-700"
									>{DateValue.format(attempt.startedAt)}</td
								>
								<td class="px-3 py-2 text-zinc-700"
									>{DateValue.format(attempt.submittedAt)}</td
								>
								<td class="px-3 py-2 text-zinc-700">
									{attempt.score !== null ? attempt.score : "-"}
								</td>
								<td class="px-3 py-2 text-zinc-700">
									{attempt.grade !== null ? GradeValue.format(attempt.grade) : "-"}
								</td>
								<td class="px-3 py-2">
									{#if attempt.warningCount > 0}
										<span
											class="inline-flex items-center justify-center rounded-full px-2 py-0.5 text-xs font-bold tabular-nums {warningBadge(attempt.warningCount)}"
										>
											{attempt.warningCount}
										</span>
									{:else}
										<span class="text-zinc-400">-</span>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>
