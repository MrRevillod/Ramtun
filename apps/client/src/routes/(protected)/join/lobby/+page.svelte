<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { browser } from "$app/environment"
	import { createMutation, createQuery } from "@tanstack/svelte-query"
	import { Play, TimerReset } from "lucide-svelte"
	import { toast } from "svelte-sonner"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { quizKindLabel } from "$lib/shared/labels"

	type AttemptSession = {
		joinCode: string
		preview: Awaited<ReturnType<typeof quizzesService.joinByCodeOrThrow>>
		attempt: Awaited<ReturnType<typeof attemptsService.initializeOrThrow>>
		answers: Record<
			string,
			{ answerIndex: number; certaintyLevel: "low" | "medium" | "high" | null }
		>
		index: number
	}

	const ATTEMPT_SESSION_KEY = "join-attempt-session"
	const joinCode = $derived(page.url.searchParams.get("joinCode") ?? "")

	const previewQuery = createQuery(() => ({
		queryKey: ["join-preview", joinCode],
		queryFn: () => quizzesService.joinByCodeOrThrow(joinCode),
		enabled: joinCode.length > 0,
	}))

	const initializeMutation = createMutation(() => ({
		mutationFn: (quizId: string) => attemptsService.initializeOrThrow(quizId),
		onSuccess: async attempt => {
			const preview = previewQuery.data
			if (!preview) return

			const session: AttemptSession = {
				joinCode,
				preview,
				attempt,
				answers: {},
				index: 0,
			}

			if (browser) {
				localStorage.setItem(ATTEMPT_SESSION_KEY, JSON.stringify(session))
			}

			await goto(resolve(`/attempts/${attempt.attemptId}`))
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
		const preview = previewQuery.data
		if (!preview) {
			preAttemptSeconds = 0
			if (preAttemptTimerId) {
				clearInterval(preAttemptTimerId)
				preAttemptTimerId = null
			}
			return
		}

		startPreAttemptCountdown(preview.startsAt)
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

	{#if previewQuery.isLoading}
		<p class="m-0 text-zinc-600">Cargando información del quiz...</p>
	{:else if previewQuery.error}
		<p class="m-0 text-red-700">{getErrorMessage(previewQuery.error)}</p>
	{:else if previewQuery.data}
		<section class="panel-elevated flex flex-col gap-4 p-5 sm:p-6 lg:min-h-[60dvh]">
			<div class="space-y-2">
				<h3 class="m-0 text-2xl text-black">{previewQuery.data.title}</h3>
				<p class="max-w-3xl text-sm leading-relaxed text-zinc-700 sm:text-base">
					Cuando inicies, responde cada pregunta y finaliza tu intento antes que
					termine el tiempo.
				</p>
			</div>

			<div class="grid gap-3 sm:grid-cols-3">
				<div class="stat-card">
					<p class="m-0 text-xs tracking-[0.16em] text-zinc-600 uppercase">Tipo</p>
					<p class="mt-2 text-lg text-black">
						{quizKindLabel(previewQuery.data.kind)}
					</p>
				</div>
				<div class="stat-card">
					<p class="m-0 text-xs tracking-[0.16em] text-zinc-600 uppercase">
						Preguntas
					</p>
					<p class="mt-2 text-lg text-black">{previewQuery.data.questionCount}</p>
				</div>
				<div class="stat-card">
					<p class="m-0 text-xs tracking-[0.16em] text-zinc-600 uppercase">
						Duracion
					</p>
					<p class="mt-2 text-lg text-black">
						{previewQuery.data.attemptDurationMinutes} min
					</p>
				</div>
			</div>

			<div
				class={previewQuery.data.kind === "certainty" &&
				previewQuery.data.certaintyTable
					? "grid gap-3 lg:grid-cols-2"
					: "grid"}
			>
				<div class="panel-muted space-y-3 p-4">
					<p class="m-0 text-sm font-medium text-black">Antes de comenzar</p>
					<ul class="space-y-1.5 pl-5 text-sm leading-relaxed text-zinc-700">
						<li>
							Inicio programado: {new Date(
								previewQuery.data.startsAt
							).toLocaleString()}
						</li>
						<li>Solo tienes un intento para este quiz.</li>
						<li>El intento se entrega automáticamente al agotarse el tiempo.</li>
					</ul>
				</div>

				{#if previewQuery.data.kind === "certainty" && previewQuery.data.certaintyTable}
					<div class="panel-muted space-y-3 p-4">
						<p class="m-0 text-sm font-medium text-black">Tabla de certeza</p>
						<div class="overflow-x-auto">
							<table class="w-full border-collapse text-xs">
								<thead class="bg-zinc-100/90 text-zinc-700">
									<tr>
										<th class="border border-zinc-300 px-2 py-1.5 text-left"
											>Nivel</th
										>
										<th class="border border-zinc-300 px-2 py-1.5 text-left"
											>Correcta</th
										>
										<th class="border border-zinc-300 px-2 py-1.5 text-left"
											>Incorrecta</th
										>
									</tr>
								</thead>
								<tbody>
									<tr>
										<td class="border border-zinc-300 bg-white px-2 py-1.5">Baja</td>
										<td class="border border-zinc-300 bg-white px-2 py-1.5"
											>{previewQuery.data.certaintyTable.low.correct}</td
										>
										<td class="border border-zinc-300 bg-white px-2 py-1.5"
											>{previewQuery.data.certaintyTable.low.incorrect}</td
										>
									</tr>
									<tr>
										<td class="border border-zinc-300 bg-white px-2 py-1.5">Media</td
										>
										<td class="border border-zinc-300 bg-white px-2 py-1.5"
											>{previewQuery.data.certaintyTable.medium.correct}</td
										>
										<td class="border border-zinc-300 bg-white px-2 py-1.5"
											>{previewQuery.data.certaintyTable.medium.incorrect}</td
										>
									</tr>
									<tr>
										<td class="border border-zinc-300 bg-white px-2 py-1.5">Alta</td>
										<td class="border border-zinc-300 bg-white px-2 py-1.5"
											>{previewQuery.data.certaintyTable.high.correct}</td
										>
										<td class="border border-zinc-300 bg-white px-2 py-1.5"
											>{previewQuery.data.certaintyTable.high.incorrect}</td
										>
									</tr>
								</tbody>
							</table>
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
					onclick={() => initializeMutation.mutate(previewQuery.data.quizId)}
					disabled={initializeMutation.isPending || isPreAttemptWait}
				>
					<Play size={14} class="mr-1 inline" />
					{#if initializeMutation.isPending}
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
