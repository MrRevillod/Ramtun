<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { browser } from "$app/environment"
	import { createMutation } from "@tanstack/svelte-query"
	import { onDestroy } from "svelte"
	import { toast } from "svelte-sonner"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import type {
		AnswerState,
		AttemptSession,
		CertaintyLevel,
	} from "$lib/attempts/attempts.dtos"
	import { getErrorMessage } from "$lib/shared/errors"
	import ProgressBar from "$lib/attempts/components/ProgressBar.svelte"
	import QuizTimer from "$lib/attempts/components/QuizTimer.svelte"
	import QuizOption from "$lib/attempts/components/QuizOption.svelte"
	import CertaintySelector from "$lib/attempts/components/CertaintySelector.svelte"
	import QuizNavigation from "$lib/attempts/components/QuizNavigation.svelte"
	import SubmitSuccessModal from "$lib/attempts/components/SubmitSuccessModal.svelte"

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
		const raw = localStorage.getItem("last-attempt-session")
		if (!raw) {
			await goto(resolve("/join"))
			return
		}

		try {
			const parsed = JSON.parse(raw) as AttemptSession
			if (parsed.attempt.attemptId !== data.attemptId) {
				await goto(resolve("/join"))
				return
			}

			session = parsed
			currentIndex = Math.max(
				0,
				Math.min(parsed.index, parsed.attempt.questions.length - 1)
			)
			answers = parsed.answers
		} catch {
			await goto("/join")
		}
	}

	void loadSession()

	const persistSession = () => {
		if (!browser || !session) return
		localStorage.setItem(
			"last-attempt-session",
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
			const diff = Math.max(
				0,
				Math.floor((new Date(expiresAt).getTime() - Date.now()) / 1000)
			)
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
				questionId: payload.questionId,
			}),
	}))

	const submitMutation = createMutation(() => ({
		mutationFn: (attemptId: string) => attemptsService.submitOrThrow(attemptId),
		onSuccess: () => {
			if (browser) {
				localStorage.removeItem("last-attempt-session")
				if (session)
					localStorage.setItem("last-submitted-join-code", session.joinCode)
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
			toast.info("Primero selecciona una alternativa.")
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

	const firstCertaintyGapIndex = () => {
		if (!session || session.preview.kind !== "certainty") return -1
		return session.attempt.questions.findIndex(question => {
			const answer = answers[question.id]
			if (!answer) return false
			return answer.certaintyLevel === null
		})
	}

	const firstUnansweredIndex = () => {
		if (!session) return -1
		return session.attempt.questions.findIndex(
			q => answers[q.id]?.answerIndex === undefined
		)
	}

	const submitAttempt = async (force = false) => {
		if (!session) return
		if (!force) {
			const unansweredIdx = firstUnansweredIndex()
			if (unansweredIdx >= 0) {
				currentIndex = unansweredIdx
				persistSession()
				toast.error("Debes responder todas las preguntas antes de enviar.")
				return
			}
			if (hasCertaintyGap()) {
				const gapIndex = firstCertaintyGapIndex()
				if (gapIndex >= 0) {
					currentIndex = gapIndex
					persistSession()
				}
				toast.error("Te falta seleccionar nivel de certeza en una o más preguntas.")
				return
			}
		}
		await submitMutation.mutateAsync(session.attempt.attemptId)
	}

	const currentQuestion = $derived(session?.attempt.questions[currentIndex] ?? null)
	const totalQuestions = $derived(session?.attempt.questions.length ?? 0)
	const currentAnswer = $derived(
		currentQuestion ? answers[currentQuestion.id] : null
	)
	const progress = $derived(
		session ? `${currentIndex + 1}/${session.attempt.questions.length}` : "0/0"
	)
</script>

{#if session && currentQuestion}
	<section class="grid gap-5">
		<section class="panel-elevated p-4 sm:p-6">
			<header class="mb-5 flex flex-wrap items-start justify-between gap-3">
				<div>
					<h3 class="m-0 text-xl text-black">{session.preview.title}</h3>
					<p class="mt-1 mb-0 text-sm text-zinc-600">Pregunta {progress}</p>
					<div class="mt-2">
						<ProgressBar current={currentIndex + 1} total={totalQuestions} />
					</div>
				</div>
				<QuizTimer {remainingSeconds} />
			</header>

			<div class="keyline"></div>
			<h4 class="mt-5 mb-0 text-lg leading-relaxed text-black">
				{currentQuestion.prompt}
			</h4>

			<div class="mt-6 grid gap-3">
				{#each currentQuestion.options as option, optionIndex (optionIndex)}
					<QuizOption
						text={option}
						isSelected={currentAnswer?.answerIndex === optionIndex}
						onclick={() => selectOption(currentQuestion.id, optionIndex)}
					/>
				{/each}
			</div>

			{#if session.preview.kind === "certainty"}
				<CertaintySelector
					selected={currentAnswer?.certaintyLevel ?? null}
					onclick={level => selectCertainty(currentQuestion.id, level)}
				/>
			{/if}

			<QuizNavigation
				{currentIndex}
				{totalQuestions}
				isLast={currentIndex >= totalQuestions - 1}
				isPending={submitMutation.isPending}
				{autoSubmitting}
				onnavigate={index => {
					const currentQuestionId = session!.attempt.questions[currentIndex].id
					const currentAnswer = answers[currentQuestionId]
					if (currentAnswer?.answerIndex === undefined) {
						toast.error("Debes seleccionar una alternativa antes de continuar.")
						return
					}
					if (
						session!.preview.kind === "certainty" &&
						!currentAnswer.certaintyLevel
					) {
						toast.error("Debes seleccionar un nivel de certeza antes de continuar.")
						return
					}
					currentIndex = index
					persistSession()
				}}
				onsubmit={() => submitAttempt()}
			/>
		</section>
	</section>
{:else}
	<p class="text-zinc-600">Cargando intento...</p>
{/if}

<SubmitSuccessModal
	open={showSubmitModal}
	joinCode={session?.joinCode ?? ""}
	onautoclose={() => (showSubmitModal = false)}
/>
