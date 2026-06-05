<script lang="ts">
	import { ShieldAlert } from "lucide-svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { onDestroy } from "svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { onAttemptWarning } from "$lib/shared/socket/attempts.socket"
	import type { WarningType } from "$lib/attempts/attempts.dtos"
	import { WARNING_LABELS } from "$lib/attempts/attempts.dtos"

	let { data } = $props()

	const warningsQuery = createQuery(() => ({
		queryKey: ["attempt-warnings", data.attemptId],
		queryFn: () => attemptsService.getAttemptWarnings(data.attemptId),
	}))

	const unsubWarning = onAttemptWarning(() => {
		void warningsQuery.refetch()
	})

	onDestroy(() => {
		unsubWarning()
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
				return "text-orange-700 bg-orange-50 border-orange-300"
			case "focus_loss":
			case "navigation":
				return "text-red-700 bg-red-50 border-red-300"
			case "devtools":
				return "text-red-800 bg-red-100 border-red-500"
		}
	}
</script>

<div class="flex items-center gap-2">
	<ShieldAlert size={18} class="text-zinc-700" />
	<h3 class="m-0 text-xl text-black">Advertencias del intento</h3>
</div>

{#if warningsQuery.isLoading}
	<p class="mt-4 mb-0 text-zinc-600">Cargando advertencias...</p>
{:else if !warningsQuery.data?.length}
	<p class="mt-4 mb-0 text-zinc-600">
		No se registraron advertencias en este intento.
	</p>
{:else}
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
						{formatDate(warning.createdAt)}
					</span>
				</div>
				<p class="mt-1 mb-0 text-sm text-zinc-700">{warning.details}</p>
			</div>
		{/each}
	</div>
{/if}
