<script lang="ts">
	import { createQuery, useQueryClient } from "@tanstack/svelte-query"
	import { toast } from "svelte-sonner"
	import { Clock, Eye, RefreshCw, X } from "lucide-svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import type { AttemptListItem, AttemptResult } from "$lib/attempts/types"
	import { getErrorMessage } from "$lib/shared/errors"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const attemptsQuery = createQuery(() => ({
		queryKey: ["attempts", "managed", data.courseId, data.quizId],
		queryFn: () =>
			attemptsService.listAttemptsOrThrow(data.courseId, data.quizId),
		refetchInterval: 15_000,
	}))

	let selectedAttempt = $state<AttemptListItem | null>(null)
	let showDetailModal = $state(false)
	let detailResult = $state<AttemptResult | null>(null)
	let detailLoading = $state(false)

	const openDetail = async (attempt: AttemptListItem) => {
		selectedAttempt = attempt
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

	const formatDatetime = (iso: string) =>
		new Date(iso).toLocaleString()

	const attemptStatus = (attempt: AttemptListItem) => {
		if (attempt.submittedAt) {
			return {
				label: attempt.resultsViewedAt ? "Revisado" : "Entregado",
				badge: attempt.resultsViewedAt
					? "bg-zinc-900 text-white"
					: "bg-amber-100 text-amber-800",
			}
		}
		return {
			label: "En curso",
			badge: "bg-emerald-100 text-emerald-800",
		}
	}
</script>

<section class="grid gap-4">
	<header>
		<p class="section-kicker m-0">Intentos</p>
		<h3 class="mt-2 mb-0 text-xl text-black">
			Intentos
		</h3>
		<p class="m-0 mt-2 text-zinc-700">
			Revisa los intentos realizados por los estudiantes en este quiz.
		</p>
	</header>

	<section class="panel-surface p-4">
		<div class="mb-3 flex items-center justify-between gap-3">
			<h4 class="m-0 text-base text-black">Listado de intentos</h4>
			<button
				class="btn-secondary flex items-center gap-1.5"
				type="button"
				onclick={() => attemptsQuery.refetch()}
				disabled={attemptsQuery.isFetching}
			>
				<RefreshCw size={16} aria-hidden="true" />
				Actualizar
			</button>
		</div>

		{#if attemptsQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando intentos...</p>
		{:else if attemptsQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(attemptsQuery.error)}</p>
		{:else if !attemptsQuery.data?.length}
			<p class="m-0 text-zinc-600">Aun no hay intentos registrados para este quiz.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="bg-zinc-100/90 text-zinc-700">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Estudiante</th>
							<th class="px-3 py-2 text-left font-medium">Inicio</th>
							<th class="px-3 py-2 text-left font-medium">Estado</th>
							<th class="px-3 py-2 text-left font-medium">Nota</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each attemptsQuery.data as attempt (attempt.attemptId)}
							{@const status = attemptStatus(attempt)}
							<tr class="border-t border-zinc-200 bg-white/80">
								<td class="px-3 py-2 text-zinc-900">
									<code class="code-chip">{attempt.studentId.slice(0, 8)}</code>
								</td>
								<td class="px-3 py-2 text-zinc-700">
									<span class="inline-flex items-center gap-1">
										<Clock size={12} aria-hidden="true" />
										{formatDatetime(attempt.startedAt)}
									</span>
								</td>
								<td class="px-3 py-2">
									<span
										class={"inline-flex items-center gap-1 rounded-[4px] px-2 py-1 text-xs " + status.badge}
									>
										{status.label}
									</span>
								</td>
								<td class="px-3 py-2 text-zinc-700">
									{#if attempt.score !== null}
										<span class="font-medium">{attempt.score} pts</span>
										{#if attempt.grade !== null}
											<span class="ml-1 text-zinc-500">({attempt.grade.toFixed(2)})</span>
										{/if}
									{:else}
										<span class="text-zinc-400">--</span>
									{/if}
								</td>
								<td class="px-3 py-2">
									{#if attempt.submittedAt}
										<button
											class="btn-secondary flex items-center gap-1.5"
											type="button"
											onclick={() => openDetail(attempt)}
										>
											<Eye size={16} aria-hidden="true" />
											Ver detalle
										</button>
									{:else}
										<span class="text-xs text-zinc-400">Sin enviar</span>
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
			class="w-full max-w-3xl max-h-[85vh] overflow-y-auto rounded-lg bg-white p-6 shadow-xl"
			role="presentation"
			tabindex="-1"
			onclick={e => {
				e.stopPropagation()
			}}
		>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="m-0 text-lg text-black">Detalle del intento</h3>
				<button
					class="btn-tertiary p-1"
					type="button"
					onclick={closeDetail}
				>
					<X size={18} aria-hidden="true" />
				</button>
			</div>

			{#if selectedAttempt}
				<p class="mt-0 mb-3 text-sm text-zinc-600">
					Estudiante: <code class="code-chip">{selectedAttempt.studentId.slice(0, 8)}</code>
					· Inicio: {formatDatetime(selectedAttempt.startedAt)}
					{#if selectedAttempt.submittedAt}
						· Entregado: {formatDatetime(selectedAttempt.submittedAt)}
					{/if}
				</p>
			{/if}

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
