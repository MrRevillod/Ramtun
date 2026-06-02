<script lang="ts">
	import { resolve } from "$app/paths"
	import { ArrowLeft, ShieldAlert } from "lucide-svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { onDestroy } from "svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import {
		onAttemptWarning,
		onAttemptsSubmit,
	} from "$lib/shared/socket/attempts.socket"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"
	import type { AttemptWarning, WarningType } from "$lib/attempts/attempts.dtos"
	import { WARNING_LABELS } from "$lib/attempts/attempts.dtos"

	let { data } = $props()

	type Tab = "results" | "supervision"
	let activeTab = $state<Tab>("results")
	let liveWarnings = $state<AttemptWarning[]>([])

	const resultsQuery = createQuery(() => ({
		queryKey: ["attempt-result", "managed", data.attemptId],
		queryFn: () => attemptsService.getAttemptResultsManaged(data.attemptId),
	}))

	const warningsQuery = createQuery(() => ({
		queryKey: ["attempt-warnings", data.attemptId],
		queryFn: () => attemptsService.getAttemptWarnings(data.attemptId),
	}))

	const unsubWarning = onAttemptWarning((warning: AttemptWarning) => {
		if (warning.attemptId === data.attemptId) {
			liveWarnings = [warning, ...liveWarnings]
		}
	})

	const unsubSubmit = onAttemptsSubmit(payload => {
		if (payload.attemptId === data.attemptId) {
			void resultsQuery.refetch()
		}
	})

	onDestroy(() => {
		unsubWarning()
		unsubSubmit()
	})

	const warningBadge = $derived(
		warningsQuery.data
			? warningsQuery.data.length + liveWarnings.length
			: liveWarnings.length
	)

	const allWarnings = $derived.by(() => {
		const ids: Record<string, boolean> = {}
		for (const w of liveWarnings) ids[w.id] = true
		const rest = (warningsQuery.data ?? []).filter(w => !ids[w.id])
		return [...liveWarnings, ...rest]
	})

	const formatDate = (value: string) =>
		new Intl.DateTimeFormat("es-CL", {
			dateStyle: "medium",
			timeStyle: "medium",
		}).format(new Date(value))

	const warningTypeColor = (type: WarningType) => {
		switch (type) {
			case "clipboard":
			case "screenshot":
				return "text-amber-700 bg-amber-50 border-amber-300"
			case "focus_loss":
			case "navigation":
				return "text-orange-700 bg-orange-50 border-orange-300"
			case "devtools":
				return "text-red-700 bg-red-50 border-red-300"
		}
	}
</script>

<section class="panel-elevated p-0 sm:p-0">
	<div
		class="flex items-center justify-between gap-4 border-b border-zinc-200 px-4 py-3 sm:px-6"
	>
		<div class="flex items-center gap-4">
			<button
				class="tab-btn"
				class:tab-active={activeTab === "results"}
				onclick={() => (activeTab = "results")}
			>
				Resultados
			</button>
			<button
				class="tab-btn"
				class:tab-active={activeTab === "supervision"}
				onclick={() => (activeTab = "supervision")}
			>
				<div class="flex items-center gap-1.5">
					Supervisión
					{#if warningBadge > 0}
						<span
							class="inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-red-600 px-1.5 text-xs font-bold text-white"
						>
							{warningBadge}
						</span>
					{/if}
				</div>
			</button>
		</div>
		<a
			class="btn-tertiary inline-flex shrink-0 items-center gap-1"
			href={resolve(`/courses/${data.courseId}/quizzes/${data.quizId}/attempts`)}
		>
			<ArrowLeft size={14} />
			Volver a intentos
		</a>
	</div>

	<div class="p-4 sm:p-6">
		{#if activeTab === "results"}
			<h3 class="mt-0 mb-4 text-xl text-black">
				{resultsQuery.data ? `Resultados de ${resultsQuery.data.userName}` : ""}
			</h3>
			{#if resultsQuery.isLoading}
				<p class="m-0 text-zinc-600">Cargando resultados...</p>
			{:else if resultsQuery.error}
				{#if getErrorMessage(resultsQuery.error).includes("no ha sido enviado")}
					<p class="m-0 text-zinc-600">
						El estudiante aún no ha enviado el intento. Los resultados estarán
						disponibles una vez que lo complete.
					</p>
				{:else}
					<p class="m-0 text-red-700">{getErrorMessage(resultsQuery.error)}</p>
				{/if}
			{:else if resultsQuery.data}
				<AttemptResultReview result={resultsQuery.data} />
			{/if}
		{:else if activeTab === "supervision"}
			<div class="flex items-center gap-2">
				<ShieldAlert size={18} class="text-zinc-700" />
				<h3 class="m-0 text-xl text-black">Advertencias del intento</h3>
			</div>

			{#if warningsQuery.isLoading && allWarnings.length === 0}
				<p class="mt-4 mb-0 text-zinc-600">Cargando advertencias...</p>
			{:else if allWarnings.length === 0}
				<p class="mt-4 mb-0 text-zinc-600">
					No se registraron advertencias en este intento.
				</p>
			{:else}
				<div class="mt-4 space-y-2">
					{#each allWarnings as warning (warning.id)}
						<div class="rounded-sm border border-zinc-200 bg-white p-3">
							<div class="flex items-center justify-between gap-3">
								<div class="flex items-center gap-2">
									<span
										class="inline-block rounded-sm border px-2 py-0.5 text-xs font-medium {warningTypeColor(
											warning.warningType
										)}"
									>
										{WARNING_LABELS[warning.warningType]}
									</span>
									<span class="text-sm text-zinc-500">#{warning.sequenceNumber}</span
									>
								</div>
								<span class="shrink-0 text-xs text-zinc-400">
									{formatDate(warning.createdAt)}
								</span>
							</div>
							<p class="mt-1 mb-0 text-sm text-zinc-700">{warning.details}</p>
						</div>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
</section>

<style>
	.tab-btn {
		border-bottom: 2px solid transparent;
		background: transparent;
		padding: 0 0.25rem 0.5rem 0.25rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: #71717a;
		transition: color 0.15s;
	}
	.tab-btn:hover {
		color: #000;
	}
	.tab-btn.tab-active {
		border-bottom-color: #000;
		color: #000;
	}
</style>
