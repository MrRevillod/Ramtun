<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { toast } from "svelte-sonner"
	import { Eye, X } from "lucide-svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import {
		disconnectAttemptsSocket,
		onAttemptsSubmit,
	} from "$lib/shared/socket/attempts.socket"
	import type { AttemptListItem, AttemptResult } from "$lib/attempts/types"
	import { getErrorMessage } from "$lib/shared/errors"
	import { GradeValue } from "$lib/shared/value-objects/grade.value"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let { data } = $props()

	const attemptsQuery = createQuery(() => ({
		queryKey: ["attempts", "managed", data.courseId, data.quizId],
		queryFn: () => attemptsService.listAttemptsOrThrow(data.courseId, data.quizId),
	}))

	// eslint-disable-next-line @typescript-eslint/no-unused-vars
	let _selectedAttempt = $state<AttemptListItem | null>(null)
	let showDetailModal = $state(false)
	let detailResult = $state<AttemptResult | null>(null)
	let detailLoading = $state(false)

	const openDetail = async (attempt: AttemptListItem) => {
		_selectedAttempt = attempt
		showDetailModal = true
		detailLoading = true
		detailResult = null

		try {
			detailResult = await attemptsService.getAttemptResultsManagedOrThrow(
				attempt.attemptId
			)
		} catch (error) {
			toast.error(getErrorMessage(error))
		} finally {
			detailLoading = false
		}
	}

	const closeDetail = () => {
		showDetailModal = false
		selectedAttempt = null
		detailResult = null
		detailLoading = false
	}

	$effect(() => {
		const unsubscribe = onAttemptsSubmit(payload => {
			if (payload.quizId !== data.quizId) return
			void attemptsQuery.refetch()
		})

		return () => {
			unsubscribe()
			disconnectAttemptsSocket()
		}
	})

	const formatDatetime = (iso: string | null) => {
		if (!iso) return "-"
		return new Date(iso).toLocaleString()
	}
</script>

<section class="grid gap-4">
	<header>
		<h3 class="mt-2 mb-0 text-xl text-black">Intentos</h3>
		<p class="m-0 mt-2 text-zinc-700">
			Revisa los intentos realizados por los estudiantes en este quiz.
		</p>
	</header>

	<section class="panel-elevated p-4">
		{#if attemptsQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando intentos...</p>
		{:else if attemptsQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(attemptsQuery.error)}</p>
		{:else if !attemptsQuery.data?.length}
			<p class="notice notice-warn m-0">
				Aún no hay intentos registrados para este quiz.
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
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each attemptsQuery.data as attempt (attempt.attemptId)}
							<tr
								class={"table-row " +
									(attempt.submittedAt ? "row-hover cursor-pointer" : "")}
								onclick={() => {
									if (attempt.submittedAt) void openDetail(attempt)
								}}
							>
								<td class="px-3 py-2 font-medium text-zinc-900"
									>{attempt.userName}</td
								>
								<td class="px-3 py-2 text-zinc-700"
									>{formatDatetime(attempt.startedAt)}</td
								>
								<td class="px-3 py-2 text-zinc-700"
									>{formatDatetime(attempt.submittedAt)}</td
								>
								<td class="px-3 py-2 text-zinc-700">
									{attempt.score !== null ? attempt.score : "-"}
								</td>
								<td class="px-3 py-2 text-zinc-700">
									{attempt.grade !== null ? GradeValue.format(attempt.grade) : "-"}
								</td>
								<td class="px-3 py-2">
									<button
										class="icon-btn disabled:cursor-not-allowed disabled:opacity-40"
										type="button"
										title={attempt.submittedAt ? "Ver resultados" : "Aún no enviado"}
										disabled={!attempt.submittedAt}
										onclick={e => {
											e.stopPropagation()
											if (attempt.submittedAt) void openDetail(attempt)
										}}
									>
										<Eye size={15} aria-hidden="true" />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>

{#if showDetailModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={closeDetail}
		onkeydown={e => {
			if (e.key === "Escape") closeDetail()
		}}
	>
		<div
			class="panel-elevated max-h-[85vh] w-full max-w-3xl overflow-y-auto p-6"
			role="presentation"
			tabindex="-1"
			onclick={e => {
				e.stopPropagation()
			}}
		>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="m-0 text-lg text-black">Detalle del intento</h3>
				<button class="btn-tertiary p-1" type="button" onclick={closeDetail}>
					<X size={18} aria-hidden="true" />
				</button>
			</div>

			{#if detailLoading}
				<p class="text-zinc-600">Cargando detalle...</p>
			{:else if detailResult}
				<AttemptResultReview result={detailResult} />
			{:else}
				<p class="text-red-700">No se pudo cargar el detalle del intento.</p>
			{/if}
		</div>
	</div>
{/if}
