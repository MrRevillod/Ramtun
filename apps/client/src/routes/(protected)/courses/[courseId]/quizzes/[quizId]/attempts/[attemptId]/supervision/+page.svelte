<script lang="ts">
	import { AlertTriangle, AlertOctagon, ShieldOff, Siren, Info } from "lucide-svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { onAttemptWarning } from "$lib/shared/socket/attempts.socket"
	import { DateValue } from "$lib/shared/value-objects/date.value"
	import {
		WARNING_LABELS,
		WARNING_SEVERITY,
		SEVERITY_GROUPS,
		type WarningType,
		type SeverityLevel,
	} from "$lib/attempts/attempts.dtos"

	let { data } = $props()

	const warningsQuery = createQuery(() => ({
		queryKey: ["attempt-warnings", data.attemptId],
		queryFn: () => attemptsService.getAttemptWarnings(data.attemptId),
	}))

	$effect(() => {
		const unsub = onAttemptWarning(() => {
			void warningsQuery.refetch()
		})
		return () => unsub()
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
			case "critica":
				return "text-red-800 bg-red-100 border-red-500"
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
			case "critica":
				return Siren
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
			case "critica":
				return "Críticas"
		}
	}

	const warningTypeColor = (type: WarningType) =>
		severityColor(WARNING_SEVERITY[type])

	const severitySummary = $derived.by(() => {
		const items = warningsQuery.data ?? []
		const counts: Record<SeverityLevel, number> = {
			leve: 0,
			moderada: 0,
			grave: 0,
			critica: 0,
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

<div class="flex items-center justify-between gap-3">
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
	<div class="mt-3 rounded-md border border-zinc-200 bg-zinc-50 p-3 text-sm">
		<p class="m-0 mb-2 text-xs font-semibold tracking-wider text-zinc-500 uppercase">
			Equivalencia de severidad
		</p>
		<div class="grid gap-2 sm:grid-cols-4">
			{#each SEVERITY_GROUPS as severity (severity)}
				{@const Icon = severityIcon(severity)}
				<div class="rounded-sm border border-zinc-200 bg-white p-2.5">
					<p
						class="m-0 flex items-center gap-1.5 text-xs font-semibold text-zinc-700"
					>
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
	<p class="mt-4 mb-0 text-zinc-600">Cargando advertencias...</p>
{:else if !warningsQuery.data?.length}
	<p class="mt-4 mb-0 text-zinc-600">
		No se registraron advertencias en este intento.
	</p>
{:else}
	<div class="mt-4 grid gap-3 sm:grid-cols-4">
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

	<div class="mt-4 space-y-2">
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
