<script lang="ts">
	import { page } from "$app/state"
	import { resolve } from "$app/paths"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { DateValue } from "$lib/shared/value-objects/date.value"
	import { quizKindLabel } from "$lib/shared/labels"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { Play, TimerReset } from "lucide-svelte"
	import { createMutation, createQuery } from "@tanstack/svelte-query"
	import type { AttemptSession } from "$lib/attempts/attempts.dtos"

	import CertaintyTableView from "$lib/quizzes/components/CertaintyTableView.svelte"

	const joinCode = $derived.by(() => page.url.searchParams.get("joinCode") ?? "")

	const previewQuery = createQuery(() => ({
		queryKey: ["join-preview", joinCode],
		queryFn: () => quizzesService.joinByCodeOrThrow(joinCode),
		enabled: joinCode.length > 0,
	}))

	const previewData = $derived.by(() => previewQuery.data)
	const isPreviewLoading = $derived.by(() => previewQuery.isLoading)
	const previewError = $derived.by(() => previewQuery.error)

	const hasCertaintyTable = $derived.by(() => {
		if (!previewData) return false
		return previewData.kind === "certainty" && !!previewData.certaintyTable
	})

	const initializeAttemptMutation = createMutation(() => ({
		mutationFn: (quizId: string) => attemptsService.initializeOrThrow(quizId),
		onSuccess: async attempt => {
			if (!previewData) {
				toast.error("No se pudo obtener la información del intento.")
				return
			}

			const session: AttemptSession = {
				joinCode,
				preview: previewData,
				attempt,
				answers: {},
				index: 0,
			}

			localStorage.setItem("last-attempt-session", JSON.stringify(session))

			await goto(`/attempts/${attempt.attemptId}`)
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const initializeAttempt = $derived.by(() => initializeAttemptMutation.mutate)
	const isInitializePending = $derived.by(() => initializeAttemptMutation.isPending)

	let preAttemptSeconds = $state(0)
	let preAttemptTimerId: ReturnType<typeof setInterval> | null = null

	const startPreAttemptCountdown = (startsAt: string) => {
		if (preAttemptTimerId) clearInterval(preAttemptTimerId)

		const update = () => {
			const diff = Math.max(
				0,
				Math.floor((new Date(startsAt).getTime() - Date.now()) / 1000)
			)

			preAttemptSeconds = diff
		}

		update()

		preAttemptTimerId = setInterval(update, 1000)
	}

	$effect(() => {
		if (!previewData) {
			preAttemptSeconds = 0
			if (preAttemptTimerId) {
				clearInterval(preAttemptTimerId)
				preAttemptTimerId = null
			}
			return
		}

		startPreAttemptCountdown(previewData.startsAt)

		return () => {
			if (preAttemptTimerId) clearInterval(preAttemptTimerId)
		}
	})

	const isPreAttemptWait = $derived(preAttemptSeconds > 0)

	const preAttemptTimerLabel = $derived.by(() => {
		const min = Math.floor(preAttemptSeconds / 60)
		const sec = preAttemptSeconds % 60
		return `${String(min).padStart(2, "0")}:${String(sec).padStart(2, "0")}`
	})
</script>

<section class="grid gap-5">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Lobby del quiz</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Revisa los detalles y confirma que estás listo antes de iniciar.
		</p>
	</header>

	{#if isPreviewLoading}
		<p class="m-0 text-zinc-600">Cargando información del quiz...</p>
	{:else if previewError}
		<p class="m-0 text-red-700">{getErrorMessage(previewError)}</p>
	{:else if previewData}
		<section class="panel-elevated flex flex-col gap-4 p-5 sm:p-6 lg:min-h-[60dvh]">
			<div class="space-y-2">
				<h3 class="m-0 text-2xl text-black">{previewData.title}</h3>
				<p class="max-w-3xl text-sm leading-relaxed text-zinc-700 sm:text-base">
					Cuando inicies, responde cada pregunta y finaliza tu intento antes que
					termine el tiempo.
				</p>
			</div>

			<div class="grid gap-3 sm:grid-cols-3">
				<div class="stat-card">
					<p class="m-0 text-xs tracking-[0.16em] text-zinc-600 uppercase">Tipo</p>
					<p class="mt-2 text-lg text-black">
						{quizKindLabel(previewData.kind)}
					</p>
				</div>
				<div class="stat-card">
					<p class="m-0 text-xs tracking-[0.16em] text-zinc-600 uppercase">
						Preguntas
					</p>
					<p class="mt-2 text-lg text-black">{previewData.questionCount}</p>
				</div>
				<div class="stat-card">
					<p class="m-0 text-xs tracking-[0.16em] text-zinc-600 uppercase">
						Duracion
					</p>
					<p class="mt-2 text-lg text-black">
						{previewData.attemptDurationMinutes} min
					</p>
				</div>
			</div>

			<div class={hasCertaintyTable ? "grid gap-3 lg:grid-cols-2" : "grid"}>
				<div class="panel-muted space-y-3 p-4">
					<p class="m-0 text-sm font-medium text-black">Antes de comenzar</p>
					<ul class="space-y-1.5 pl-5 text-sm leading-relaxed text-zinc-700">
						<li>
							Inicio programado: {DateValue.format(previewData.startsAt)}
						</li>
						<li>Solo tienes un intento para este quiz.</li>
						<li>El intento se entrega automáticamente al agotarse el tiempo.</li>
					</ul>
				</div>

				{#if hasCertaintyTable}
					<div class="panel-muted space-y-3 p-4">
						<p class="m-0 text-sm font-medium text-black">Tabla de certeza</p>
						<div class="overflow-x-auto">
							<CertaintyTableView table={previewData.certaintyTable!} />
						</div>
					</div>
				{/if}
			</div>

			<div class="keyline"></div>
			<div class="mt-auto flex flex-wrap items-center justify-between gap-3">
				<a class="btn-tertiary" href={resolve("/join")}>
					<TimerReset size={14} class="mr-1 inline" />
					Cambiar código
				</a>
				<button
					class="btn-primary"
					type="button"
					onclick={() => initializeAttempt(previewData.quizId)}
					disabled={isInitializePending || isPreAttemptWait}
				>
					<Play size={14} class="mr-1 inline" />
					{#if isInitializePending}
						Iniciando...
					{:else if isPreAttemptWait}
						Comienza en {preAttemptTimerLabel}
					{:else}
						Comenzar intento
					{/if}
				</button>
			</div>
		</section>
	{/if}
</section>
