<script lang="ts">
	import { goto } from "$app/navigation"
	import { browser } from "$app/environment"
	import { createMutation } from "@tanstack/svelte-query"
	import { onDestroy } from "svelte"
	import { toast } from "svelte-sonner"
	import {
		Clock,
		ArrowRight,
		Send,
		X,
		CheckCircle2,
		ArrowUpRight,
	} from "lucide-svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import type { AttemptView, CertaintyLevel } from "$lib/attempts/types"
	import type { JoinQuizPreview } from "$lib/quizzes/types"
	import { getErrorMessage } from "$lib/shared/errors"
	import { certaintyLevelLabel } from "$lib/shared/labels"

	type AnswerState = {
		answerIndex: number
		certaintyLevel: CertaintyLevel | null
	}

	type AttemptSession = {
		joinCode: string
		preview: JoinQuizPreview
		attempt: AttemptView
		answers: Record<string, AnswerState>
		index: number
	}

	const ATTEMPT_SESSION_KEY = "join-attempt-session"

	let { data } = $props()
	let session = $state<AttemptSession | null>(null)
	let currentIndex = $state(0)
	let answers = $state<Record<string, AnswerState>>({})
	let remainingSeconds = $state(0)
	let showSubmitModal = $state(false)
	let autoSubmitting = $state(false)
	let timerId: ReturnType<typeof setInterval> | null = null

	const loadSession = async () => {
		if (!browser) return
		const raw = localStorage.getItem(ATTEMPT_SESSION_KEY)
		if (!raw) {
			await goto("/join")
			return
		}

		try {
			const parsed = JSON.parse(raw) as AttemptSession
			if (parsed.attempt.attemptId !== data.attemptId) {
				await goto("/join")
				return
			}

			session = parsed
			currentIndex = Math.max(0, Math.min(parsed.index, parsed.attempt.questions.length - 1))
			answers = parsed.answers
		} catch {
			await goto("/join")
		}
	}

	void loadSession()

	const persistSession = () => {
		if (!browser || !session) return
		localStorage.setItem(
			ATTEMPT_SESSION_KEY,
			JSON.stringify({
				...session,
				index: currentIndex,
				answers,
			})
		)
	}

	const clearTimer = () => {
		if (timerId) {
			clearInterval(timerId)
			timerId = null
		}
	}

	const startTimer = (expiresAt: string) => {
		clearTimer()
		const update = () => {
			const diff = Math.max(0, Math.floor((new Date(expiresAt).getTime() - Date.now()) / 1000))
			remainingSeconds = diff
			if (diff === 0 && !autoSubmitting && !submitMutation.isPending) {
				autoSubmitting = true
				void submitAttempt(true).finally(() => {
					autoSubmitting = false
				})
			}
		}
		update()
		timerId = setInterval(update, 1000)
	}

	$effect(() => {
		if (!session) return
		startTimer(session.attempt.expiresAt)
		return () => clearTimer()
	})

	onDestroy(clearTimer)

	const saveAnswerMutation = createMutation(() => ({
		mutationFn: (payload: {
			attemptId: string
			questionId: string
			answerIndex: number
			certaintyLevel: CertaintyLevel | null
		}) =>
			attemptsService.saveAnswerOrThrow(payload.attemptId, payload.questionId, {
				answerIndex: payload.answerIndex,
				certaintyLevel: payload.certaintyLevel,
			}),
	}))

	const submitMutation = createMutation(() => ({
		mutationFn: (attemptId: string) => attemptsService.submitOrThrow(attemptId),
		onSuccess: submitted => {
			if (browser) {
				localStorage.removeItem(ATTEMPT_SESSION_KEY)
				localStorage.setItem("last-submitted-attempt-id", submitted.attemptId)
				if (session) localStorage.setItem("last-submitted-join-code", session.joinCode)
			}
			clearTimer()
			showSubmitModal = true
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const runSave = async (questionId: string) => {
		if (!session) return
		const answer = answers[questionId]
		if (!answer) return

		try {
			await saveAnswerMutation.mutateAsync({
				attemptId: session.attempt.attemptId,
				questionId,
				answerIndex: answer.answerIndex,
				certaintyLevel: answer.certaintyLevel,
			})
			persistSession()
		} catch {
			// handled by mutation onError
		}
	}

	const selectOption = async (questionId: string, answerIndex: number) => {
		const prev = answers[questionId]
		answers = {
			...answers,
			[questionId]: {
				answerIndex,
				certaintyLevel: prev?.certaintyLevel ?? null,
			},
		}
		await runSave(questionId)
	}

	const selectCertainty = async (questionId: string, certainty: CertaintyLevel) => {
		const prev = answers[questionId]
		if (!prev) {
			toast.error("Primero selecciona una alternativa.")
			return
		}
		answers = {
			...answers,
			[questionId]: { answerIndex: prev.answerIndex, certaintyLevel: certainty },
		}
		await runSave(questionId)
	}

	const hasCertaintyGap = () => {
		if (!session || session.preview.kind !== "certainty") return false
		return session.attempt.questions.some(question => {
			const answer = answers[question.id]
			if (!answer) return false
			return answer.certaintyLevel === null
		})
	}

	const submitAttempt = async (force = false) => {
		if (!session) return
		if (!force && hasCertaintyGap()) {
			toast.error("Debes seleccionar nivel de certeza antes de finalizar.")
			return
		}
		await submitMutation.mutateAsync(session.attempt.attemptId)
	}

	const currentQuestion = $derived(session?.attempt.questions[currentIndex] ?? null)
	const totalQuestions = $derived(session?.attempt.questions.length ?? 0)
	const currentAnswer = $derived(currentQuestion ? answers[currentQuestion.id] : null)
	const progress = $derived(
		session ? `${currentIndex + 1}/${session.attempt.questions.length}` : "0/0"
	)
	const timerLabel = $derived.by(() => {
		const min = Math.floor(remainingSeconds / 60)
		const sec = remainingSeconds % 60
		return `${String(min).padStart(2, "0")}:${String(sec).padStart(2, "0")}`
	})
</script>

{#if session && currentQuestion}
	<section class="grid gap-5">
		<header class="panel-muted p-4 sm:p-5">
			<div class="flex flex-wrap items-center justify-between gap-2">
				<div>
					<p class="m-0 text-sm text-zinc-600">Progreso: {progress}</p>
					<h3 class="m-0 mt-1 text-xl text-black">{session.preview.title}</h3>
				</div>
				<p class="m-0 text-sm font-medium text-zinc-800">
					<Clock size={14} class="-mt-0.5 mr-0.5 inline-block" aria-hidden="true" />
					{timerLabel}
				</p>
			</div>
		</header>

		<section class="panel-surface p-4 sm:p-6">
			<h4 class="m-0 text-lg text-black">{currentQuestion.prompt}</h4>

			<div class="mt-6 grid gap-3">
				{#each currentQuestion.options as option, optionIndex}
					<button
						type="button"
						class={currentAnswer?.answerIndex === optionIndex
							? "btn-primary w-full justify-start py-3"
							: "btn-secondary w-full justify-start py-3"}
						onclick={() => selectOption(currentQuestion.id, optionIndex)}
					>
						{option}
					</button>
				{/each}
			</div>

			{#if session.preview.kind === "certainty"}
				<div class="mt-7 grid gap-2 sm:grid-cols-3">
					{#each ["low", "medium", "high"] as level}
						<button
							type="button"
							class={currentAnswer?.certaintyLevel === level ? "btn-primary" : "btn-tertiary"}
							onclick={() => selectCertainty(currentQuestion.id, level as CertaintyLevel)}
						>
							Certeza {certaintyLevelLabel(level as CertaintyLevel)}
						</button>
					{/each}
				</div>
			{/if}

			<div class="mt-7 flex flex-wrap justify-end gap-2">
				{#if currentIndex < totalQuestions - 1}
					<button
						class="btn-secondary flex items-center gap-1.5"
						type="button"
						onclick={() => {
							currentIndex = Math.min(totalQuestions - 1, currentIndex + 1)
							persistSession()
						}}
					>
						Siguiente
						<ArrowRight size={16} aria-hidden="true" />
					</button>
				{:else}
					<button class="btn-primary flex items-center gap-1.5" type="button" onclick={() => submitAttempt()}>
						<Send size={16} aria-hidden="true" />
						{submitMutation.isPending || autoSubmitting ? "Enviando..." : "Finalizar intento"}
					</button>
				{/if}
			</div>
		</section>
	</section>
{:else}
	<p class="text-zinc-600">Cargando intento...</p>
{/if}

{#if showSubmitModal}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={() => (showSubmitModal = false)}
		onkeydown={e => {
			if (e.key === "Escape") showSubmitModal = false
		}}
	>
		<div class="panel-surface w-full max-w-md p-6" role="presentation" tabindex="-1" onclick={e => e.stopPropagation()}>
			<div class="mb-4 flex items-center justify-between">
				<h3 class="m-0 flex items-center gap-2 text-xl text-black">
					<CheckCircle2 size={20} class="text-emerald-600" aria-hidden="true" />
					Intento enviado
				</h3>
				<button class="btn-tertiary p-1" type="button" onclick={() => (showSubmitModal = false)}>
					<X size={18} aria-hidden="true" />
				</button>
			</div>
			<p class="mb-4 text-sm text-zinc-700">
				Tu intento fue enviado correctamente. Los resultados estaran disponibles cuando el docente los publique.
			</p>
			<div class="grid gap-2">
				<button class="btn-primary flex items-center justify-center gap-1.5" type="button" onclick={() => goto(`/results/lobby?joinCode=${encodeURIComponent(session ? session.joinCode : "")}`)}>
					<ArrowUpRight size={16} aria-hidden="true" />
					Ir a sala de espera de resultados
				</button>
				<button class="btn-secondary" type="button" onclick={() => goto("/join")}>Volver a unirse</button>
			</div>
		</div>
	</div>
{/if}
