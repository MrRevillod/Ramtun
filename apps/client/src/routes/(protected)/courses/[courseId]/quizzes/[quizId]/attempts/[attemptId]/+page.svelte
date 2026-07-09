<script lang="ts">
	import { ArrowLeft, FileText, Shield } from "lucide-svelte"
	import { AlertTriangle, AlertOctagon, ShieldOff, Info } from "lucide-svelte"
	import { useQuery } from "$lib/shared/http/tanstack"
	import { resolve } from "$app/paths"
	import { attemptsService } from "$lib/attempts/service"
	import { onAttemptWarning, onAttemptsSubmit } from "$lib/shared/socket/attempts.socket"
	import { DateValue } from "$lib/shared/value-objects/date.value"
	import PageHeader from "$lib/shared/components/PageHeader.svelte"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"
	import {
		WARNING_LABELS,
		WARNING_SEVERITY,
		SEVERITY_GROUPS,
		type WarningType,
		type SeverityLevel,
	} from "$lib/attempts/dtos"

	let { data } = $props()

	let activeTab = $state<"results" | "supervision">("results")

	const warningsQuery = useQuery(() => ({
		queryKey: ["attempt-warnings", data.attemptId],
		queryFn: () => attemptsService.getAttemptWarnings(data.attemptId),
	}))

	const warningBadge = $derived(warningsQuery.data?.length ?? 0)

	const resultsQuery = useQuery(() => ({
		queryKey: ["attempt-result", "managed", data.attemptId],
		queryFn: () => attemptsService.getAttemptResultsManaged(data.attemptId),
	}))

	const resultsErrorMessage = $derived(resultsQuery.error?.messageOrDefault ?? "")

	$effect(() => {
		const unsubWarning = onAttemptWarning(() => {
			void warningsQuery.refetch()
		})
		const unsubSubmit = onAttemptsSubmit((payload) => {
			if (payload.attemptId === data.attemptId) {
				void resultsQuery.refetch()
			}
		})
		return () => {
			unsubWarning()
			unsubSubmit()
		}
	})

	let showInfo = $state(false)

	const severityColor = (severity: SeverityLevel) => {
		switch (severity) {
			case "leve":
				return "text-yellow-700 bg-yellow-50 border-yellow-300"
			case "moderada":
				return "text-orange-700 bg-orange-50 border-orange-300"
			case "grave":
				return "text-red-700 bg-red-50 border-red-300"
		}
	}

	const severityIcon = (severity: SeverityLevel) => {
		switch (severity) {
			case "leve":
				return AlertTriangle
			case "moderada":
				return AlertOctagon
			case "grave":
				return ShieldOff
		}
	}

	const severityLabel = (severity: SeverityLevel) => {
		switch (severity) {
			case "leve":
				return "Leves"
			case "moderada":
				return "Moderadas"
			case "grave":
				return "Graves"
		}
	}

	const warningTypeColor = (type: WarningType) => severityColor(WARNING_SEVERITY[type])

	const severitySummary = $derived.by(() => {
		const items = warningsQuery.data ?? []
		const counts: Record<SeverityLevel, number> = {
			leve: 0,
			moderada: 0,
			grave: 0,
		}
		for (const w of items) {
			const s = WARNING_SEVERITY[w.warningType]
			if (s) counts[s]++
		}
		return counts
	})

	const severityTypes = (severity: SeverityLevel): WarningType[] =>
		(Object.entries(WARNING_SEVERITY) as [WarningType, SeverityLevel][])
			.filter(([, s]) => s === severity)
			.map(([type]) => type)
</script>

<section class="flex flex-col">
	<PageHeader supra="Revisión y Resultados" title={data.quizName} tag="h3">
		<button
			class="action-tab justify-center"
			data-active={activeTab === "results"}
			onclick={() => (activeTab = "results")}
			type="button"
		>
			<FileText size={16} /> Resultados
		</button>
		<button
			class="action-tab justify-center"
			data-active={activeTab === "supervision"}
			onclick={() => (activeTab = "supervision")}
			type="button"
		>
			<div class="flex items-center gap-1.5">
				<Shield size={16} /> Supervisión
				{#if warningBadge > 0}
					<span
						class="inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-red-600 px-1.5 text-xs font-bold text-white"
					>
						{warningBadge}
					</span>
				{/if}
			</div>
		</button>
		<a
			class="action-tab shrink-0 justify-center"
			href={resolve(`/courses/${data.courseId}/quizzes/${data.quizId}/attempts`)}
		>
			<ArrowLeft size={16} aria-hidden="true" />
			Volver
		</a>
	</PageHeader>

	{#if activeTab === "results"}
		{#if resultsQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando resultados...</p>
		{:else if resultsQuery.error}
			{#if resultsErrorMessage.includes("no ha sido enviado")}
				<p class="m-0 text-zinc-600">
					El estudiante aún no ha enviado el intento. Los resultados estarán disponibles una vez que
					lo complete.
				</p>
			{:else}
				<p class="m-0 text-red-700">{resultsErrorMessage}</p>
			{/if}
		{:else if resultsQuery.data}
			<AttemptResultReview result={resultsQuery.data} />
		{/if}
	{:else}
		<div class="mb-2 flex items-center justify-between gap-3">
			<h3 class="m-0 text-xl text-black">Advertencias del intento</h3>
			<button
				class="inline-flex size-8 cursor-pointer items-center justify-center rounded-md border border-zinc-300 bg-white text-zinc-500 transition-colors hover:border-zinc-800 hover:text-zinc-800"
				type="button"
				onclick={() => (showInfo = !showInfo)}
				title="Ver equivalencia de severidad"
			>
				<Info size={16} />
			</button>
		</div>

		{#if showInfo}
			<div class="mb-2 gap-3 rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm">
				<p class="m-0 mb-2 text-xs font-semibold tracking-wider text-zinc-500 uppercase">
					Equivalencia de severidad
				</p>
				<div class="grid gap-2 sm:grid-cols-3">
					{#each SEVERITY_GROUPS as severity (severity)}
						{@const Icon = severityIcon(severity)}
						<div class="rounded-sm border border-zinc-200 bg-white p-2.5">
							<p class="m-0 flex items-center gap-1.5 text-xs font-semibold text-zinc-700">
								<Icon size={13} />
								{severityLabel(severity)}
							</p>
							<ul class="m-0 mt-1 list-none space-y-0.5 p-0">
								{#each severityTypes(severity) as type (type)}
									<li class="text-xs text-zinc-600">{WARNING_LABELS[type]}</li>
								{/each}
							</ul>
						</div>
					{/each}
				</div>
			</div>
		{/if}

		{#if warningsQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando advertencias...</p>
		{:else if !warningsQuery.data?.length}
			<p class="m-0 text-zinc-600">No se registraron advertencias en este intento.</p>
		{:else}
			<div class="mb-2 grid gap-3 sm:grid-cols-3">
				{#each SEVERITY_GROUPS as severity (severity)}
					{@const Icon = severityIcon(severity)}
					<div class="panel-muted p-3">
						<p class="m-0 flex items-center gap-2 text-xs font-medium text-zinc-700">
							<Icon size={13} />
							{severityLabel(severity)}
						</p>
						<p class="mt-1 mb-0 text-lg font-semibold text-black">
							{severitySummary[severity]}
						</p>
					</div>
				{/each}
			</div>

			<div class="space-y-2">
				{#each warningsQuery.data as warning (warning.id)}
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
								<span class="text-sm text-zinc-500">#{warning.sequenceNumber}</span>
							</div>
							<span class="shrink-0 text-xs text-zinc-400">
								{DateValue.format(warning.createdAt, "medium")}
							</span>
						</div>
						<p class="mt-1 mb-0 text-sm text-zinc-700">{warning.details}</p>
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</section>
