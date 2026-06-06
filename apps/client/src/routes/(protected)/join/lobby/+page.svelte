<script lang="ts">
	import { page } from "$app/state"
	import { resolve } from "$app/paths"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { DateValue } from "$lib/shared/value-objects/date.value"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { Play, TimerReset } from "lucide-svelte"
	import { createMutation, createQuery } from "@tanstack/svelte-query"
	import type { AttemptSession } from "$lib/attempts/attempts.dtos"

	import CertaintyTableView from "$lib/quizzes/components/CertaintyTableView.svelte"
	import { QuizKindValue } from "$lib/shared/value-objects/quiz-kind.values"

	const joinCode = $derived.by(() => page.url.searchParams.get("joinCode") ?? "")

	const previewQuery = createQuery(() => ({
		queryKey: ["join-preview", joinCode],
		queryFn: () => quizzesService.joinByCode(joinCode),
		enabled: joinCode.length > 0,
	}))

	const hasCertaintyTable = $derived.by(() => {
		if (!previewQuery.data) return false
		return (
			previewQuery.data.kind === "certainty" && !!previewQuery.data.certaintyTable
		)
	})

	const initializeAttemptMutation = createMutation(() => ({
		mutationFn: () => attemptsService.initialize(previewQuery.data!.quizId),
		onSuccess: async attempt => {
			if (!previewQuery.data) {
				toast.error("No se pudo obtener la información del intento.")
				return
			}

			const session: AttemptSession = {
				joinCode,
				preview: previewQuery.data,
				attempt,
				answers: {},
				index: 0,
			}

			localStorage.setItem("last-attempt-session", JSON.stringify(session))

			await goto(`/attempts/${attempt.attemptId}`)
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

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
		if (!previewQuery.data) {
			preAttemptSeconds = 0

			if (preAttemptTimerId) {
				clearInterval(preAttemptTimerId)
				preAttemptTimerId = null
			}

			return
		}

		const raw = localStorage.getItem("last-attempt-session")
		if (raw) {
			try {
				const session = JSON.parse(raw) as AttemptSession
				const now = Date.now()
				const expiresAt = new Date(session.attempt.expiresAt).getTime()

				if (
					!session.attempt.submittedAt &&
					expiresAt >= now - 60000 &&
					session.preview.quizId === previewQuery.data.quizId
				) {
					void goto(`/attempts/${session.attempt.attemptId}`)
					return
				}
			} catch {
				// ignore corrupt data
			}
		}

		startPreAttemptCountdown(previewQuery.data.startsAt)

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

<section class="flex flex-col gap-4 py-4">
	<div>
		<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">
			Unirse a un cuestionario
		</p>
		{#if previewQuery.data}
			<h2 class="mt-0.5 mb-0 text-2xl tracking-tight text-black">
				{previewQuery.data.title}
			</h2>
		{:else}
			<h2 class="mt-0.5 mb-0 text-2xl tracking-tight text-black">
				Sala de espera del cuestionario
			</h2>
		{/if}
	</div>

	{#if previewQuery.isLoading}
		<p class="m-0 text-zinc-600">Cargando información del cuestionario...</p>
	{:else if previewQuery.isError}
		<p class="m-0 text-red-700">{getErrorMessage(previewQuery.error)}</p>
	{:else if previewQuery.data}
		<section class="flex flex-col gap-3">
			<div class="grid gap-2 sm:grid-cols-3">
				<div class="panel-muted p-3">
					<p class="m-0 text-xs text-zinc-600">Tipo</p>
					<p class="mt-1 mb-0 text-lg font-semibold text-black">
						{QuizKindValue.format(previewQuery.data.kind)}
					</p>
				</div>
				<div class="panel-muted p-3">
					<p class="m-0 text-xs text-zinc-600">Preguntas</p>
					<p class="mt-1 mb-0 text-lg font-semibold text-black">
						{previewQuery.data.questionCount}
					</p>
				</div>
				<div class="panel-muted p-3">
					<p class="m-0 text-xs text-zinc-600">Duración</p>
					<p class="mt-1 mb-0 text-lg font-semibold text-black">
						{previewQuery.data.attemptDurationMinutes} min
					</p>
				</div>
			</div>

			<div class={hasCertaintyTable ? "grid gap-3 lg:grid-cols-2" : "grid"}>
				<div class="panel-muted space-y-1 p-3 text-sm leading-relaxed text-zinc-700">
					<p class="m-0 text-sm font-medium text-black">Antes de comenzar</p>
					<p class="m-0">
						Inicio programado: {DateValue.format(previewQuery.data.startsAt)}
					</p>
					<p class="m-0">
						Solo tienes un intento. Se entrega automáticamente al agotarse el tiempo.
					</p>
				</div>

				{#if hasCertaintyTable}
					<div class="panel-muted p-3">
						<p class="m-0 mb-2 text-sm font-medium text-black">Tabla de certeza</p>
						<div class="overflow-x-auto">
							<CertaintyTableView table={previewQuery.data.certaintyTable!} />
						</div>
					</div>
				{/if}
			</div>

			<div class="mb-4 h-px bg-zinc-400/70"></div>
			<div class="flex flex-wrap items-center justify-between gap-3">
				<a class="action-tab justify-center" href={resolve("/join")}>
					<TimerReset size={16} /> Cambiar código
				</a>
				<button
					class="btn-primary"
					type="button"
					onclick={() => initializeAttemptMutation.mutate()}
					disabled={initializeAttemptMutation.isPending || isPreAttemptWait}
				>
					<Play size={14} class="mr-1 inline" />
					{#if initializeAttemptMutation.isPending}
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
