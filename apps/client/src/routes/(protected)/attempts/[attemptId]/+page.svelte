<script lang="ts">
	import { goto } from "$app/navigation"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"
	import { onDestroy } from "svelte"
	import { toast } from "svelte-sonner"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import type {
		AnswerState,
		CertaintyLevel,
		WarningType,
	} from "$lib/attempts/attempts.dtos"
	import { useAttempt } from "$lib/attempts/attempts.queries"
	import { getErrorMessage } from "$lib/shared/errors"
	import { createAntiCheat } from "$lib/attempts/services/anti-cheat.service.svelte"
	import ProgressBar from "$lib/attempts/components/ProgressBar.svelte"
	import QuizTimer from "$lib/attempts/components/QuizTimer.svelte"
	import QuizOption from "$lib/attempts/components/QuizOption.svelte"
	import CertaintySelector from "$lib/attempts/components/CertaintySelector.svelte"
	import QuizNavigation from "$lib/attempts/components/QuizNavigation.svelte"
	import SubmitSuccessModal from "$lib/attempts/components/SubmitSuccessModal.svelte"

	let { data } = $props()

	const attemptId = $derived(data.attemptId)

	const attemptQuery = useAttempt(() => attemptId)
	const queryClient = useQueryClient()

	let currentIndex = $state(0)
	let answers = $state<Record<string, AnswerState>>({})
	let remainingSeconds = $state(0)
	let showSubmitModal = $state(false)
	let autoSubmitting = $state(false)
	let timerId: ReturnType<typeof setInterval> | null = null

	$effect(() => {
		const attempt = attemptQuery.data
		if (!attempt) return

		const savedAnswers: Record<string, AnswerState> = {}
		for (const a of attempt.answers) {
			savedAnswers[a.questionId] = {
				answerIndex: a.answerIndex,
				certaintyLevel: a.certaintyLevel,
			}
		}
		answers = savedAnswers

		const firstUnanswered = attempt.questions.findIndex(
			q => savedAnswers[q.id]?.answerIndex === undefined
		)
		currentIndex = firstUnanswered >= 0 ? firstUnanswered : 0
	})

	const onWarning = (_type: WarningType, details: string) => {
		toast.warning(details, { duration: 4000 })
	}

	let antiCheat: ReturnType<typeof createAntiCheat> | null = null

	$effect(() => {
		if (!attemptQuery.data) return

		antiCheat = createAntiCheat(attemptQuery.data.attemptId, onWarning)
		antiCheat.start()

		return () => {
			antiCheat?.stop()
			antiCheat = null
		}
	})

	const isCertaintyQuiz = () => attemptQuery.data?.kind === "certainty"
	const isAnswered = (answer?: AnswerState | null) =>
		answer?.answerIndex !== undefined
	const isCertaintyComplete = (answer?: AnswerState | null) =>
		!!answer && answer.certaintyLevel !== null
	const isAnswerComplete = (answer?: AnswerState | null) => {
		if (!isAnswered(answer)) return false
		if (isCertaintyQuiz()) return isCertaintyComplete(answer)
		return true
	}
	const firstUnansweredIndex = () => {
		if (!attemptQuery.data) return -1
		return attemptQuery.data.questions.findIndex(
			q => answers[q.id]?.answerIndex === undefined
		)
	}
	const firstCertaintyGapIndex = () => {
		if (!attemptQuery.data || !isCertaintyQuiz()) return -1
		return attemptQuery.data.questions.findIndex(q => {
			const answer = answers[q.id]
			return !!answer && answer.certaintyLevel === null
		})
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
		if (!attemptQuery.data) return
		startTimer(attemptQuery.data.expiresAt)
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
			attemptsService.saveAnswer(payload.attemptId, payload.questionId, {
				answerIndex: payload.answerIndex,
				certaintyLevel: payload.certaintyLevel,
				questionId: payload.questionId,
			}),
	}))

	const submitMutation = createMutation(() => ({
		mutationFn: (attemptId: string) => attemptsService.submit(attemptId),
		onSuccess: () => {
			queryClient.setQueryData(["attempts", "active"], null)
			queryClient.invalidateQueries({ queryKey: ["attempts"] })
			clearTimer()
			showSubmitModal = true
			toast.success("Intento enviado correctamente.")
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const runSave = async (questionId: string) => {
		if (!attemptQuery.data) return
		const answer = answers[questionId]
		if (saveAnswerMutation.isPending) return
		if (!isAnswerComplete(answer)) return

		try {
			await saveAnswerMutation.mutateAsync({
				attemptId: attemptQuery.data.attemptId,
				questionId,
				answerIndex: answer.answerIndex,
				certaintyLevel: answer.certaintyLevel,
			})
		} catch {
			// handled by mutation onError
		}
	}

	const selectOption = (questionId: string, answerIndex: number) => {
		if (!attemptQuery.data) return
		const prev = answers[questionId]
		const nextAnswer = {
			answerIndex,
			certaintyLevel: prev?.certaintyLevel ?? null,
		}
		answers = {
			...answers,
			[questionId]: nextAnswer,
		}
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
	}

	const submitAttempt = async (force = false) => {
		if (!attemptQuery.data) return
		if (!force) {
			const unansweredIdx = firstUnansweredIndex()
			if (unansweredIdx >= 0) {
				currentIndex = unansweredIdx
				toast.error("Debes responder todas las preguntas antes de enviar.")
				return
			}
			if (isCertaintyQuiz()) {
				const gapIndex = firstCertaintyGapIndex()
				if (gapIndex >= 0) {
					currentIndex = gapIndex
					toast.error(
						"Te falta seleccionar nivel de certeza en una o más preguntas."
					)
					return
				}
			}
		}

		if (submitMutation.isPending) return
		await submitMutation.mutateAsync(attemptQuery.data.attemptId)
	}

	const currentQuestion = $derived(
		attemptQuery.data?.questions[currentIndex] ?? null
	)
	const totalQuestions = $derived(attemptQuery.data?.questions.length ?? 0)
	const currentAnswer = $derived(
		currentQuestion ? answers[currentQuestion.id] : null
	)
	const progress = $derived(
		attemptQuery.data
			? `${currentIndex + 1}/${attemptQuery.data.questions.length}`
			: "0/0"
	)
</script>

{#if attemptQuery.isLoading}
	<section class="grid gap-5">
		<section>
			<h3 class="m-0 text-xl text-black">Cargando intento...</h3>
			<p class="mt-2 text-zinc-600">Preparando pregunta...</p>
		</section>
	</section>
{:else if attemptQuery.isError || !attemptQuery.data}
	<section class="grid gap-5">
		<section>
			<h3 class="m-0 text-xl text-black">Error</h3>
			<p class="mt-2 text-zinc-600">No se pudo cargar el intento.</p>
		</section>
	</section>
{:else if currentQuestion}
	<section class="grid gap-5">
		<section class="py-6">
			<header class="mb-5 flex flex-wrap items-start justify-between gap-3">
				<div>
					<h3 class="m-0 text-xl text-black">
						{attemptQuery.data.title}
					</h3>
					<p class="mt-1 mb-0 text-sm text-zinc-600">Pregunta {progress}</p>
					<div class="mt-2">
						<ProgressBar current={currentIndex + 1} total={totalQuestions} />
					</div>
				</div>
				<QuizTimer {remainingSeconds} />
			</header>

			<div class="h-px bg-zinc-300/70"></div>
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

			{#if attemptQuery.data.kind === "certainty"}
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
				onnavigate={async index => {
					const currentQuestionId = attemptQuery.data!.questions[currentIndex].id
					const currentAns = answers[currentQuestionId]
					if (!isAnswered(currentAns)) {
						toast.error("Debes seleccionar una alternativa antes de continuar.")
						return
					}
					if (!isAnswerComplete(currentAns)) {
						toast.error("Debes seleccionar un nivel de certeza antes de continuar.")
						return
					}
					await runSave(currentQuestionId)
					currentIndex = index
				}}
				onsubmit={async () => {
					if (currentQuestion) {
						await runSave(currentQuestion.id)
					}
					await submitAttempt()
				}}
			/>
		</section>
	</section>
{/if}

<SubmitSuccessModal open={showSubmitModal} onContinue={() => goto("/results")} />
