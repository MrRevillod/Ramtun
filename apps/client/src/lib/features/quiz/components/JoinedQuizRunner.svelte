<script lang="ts">
	import { onMount } from 'svelte'
	import { createMutation } from '@tanstack/svelte-query'
	import { toast } from 'svelte-sonner'
	import { ClipboardCheck, ChevronRight, Send } from 'lucide-svelte'
	import { attemptService } from '$lib/features/attempt/attempt.service'
	import { quizUiStore } from '$lib/features/quiz/quiz.store.svelte'
	import { toUserMessage } from '$lib/shared/errors'
	import type { AttemptCertaintyLevel } from '$lib/features/quiz/types'

	const activeAttempt = $derived(quizUiStore.activeAttempt)
	const activeAttemptId = $derived(activeAttempt?.attemptId ?? null)
	const activeQuiz = $derived(activeAttempt?.quiz ?? null)
	const currentQuestion = $derived.by(() => {
		if (!activeQuiz) {
			return null
		}

		return activeQuiz.questions[quizUiStore.currentQuestionIndex] ?? null
	})

	const selectedAnswer = $derived.by(() => {
		if (!currentQuestion || !activeAttempt) {
			return undefined
		}

		return activeAttempt.answers.find((answer) => answer.questionId === currentQuestion.questionId)?.answerIndex
	})

	const totalQuestions = $derived(activeQuiz?.questions.length ?? 0)
	const answeredCount = $derived(activeAttempt?.answers.length ?? 0)
	const isLastQuestion = $derived(quizUiStore.currentQuestionIndex >= totalQuestions - 1)
	let now = $state(Date.now())
	const progress = $derived.by(() => {
		if (!totalQuestions) {
			return 0
		}

		return Math.round((answeredCount / totalQuestions) * 100)
	})
	const remainingMs = $derived.by(() => {
		if (!activeAttempt) {
			return 0
		}

		return Math.max(new Date(activeAttempt.expiresAt).getTime() - now, 0)
	})
	const remainingLabel = $derived.by(() => {
		const totalSeconds = Math.floor(remainingMs / 1000)
		const minutes = Math.floor(totalSeconds / 60)
		const seconds = totalSeconds % 60

		return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
	})
	const isExpired = $derived(remainingMs <= 0)

	let autoSubmitStarted = $state(false)
	let saveInFlightByQuestionId = $state<Record<string, boolean>>({})

	onMount(() => {
		const interval = window.setInterval(() => {
			now = Date.now()
		}, 1000)

		return () => {
			window.clearInterval(interval)
		}
	})

	$effect(() => {
		activeAttemptId
		autoSubmitStarted = false
	})

	const submitMutation = createMutation(() => ({
		mutationFn: (attemptId: string) => attemptService.submitAttempt(attemptId)
	}))

	const saveAnswer = async (
		questionId: string,
		answerIndex: number,
		certaintyLevel?: AttemptCertaintyLevel | null
	) => {
		if (!activeAttempt || !activeQuiz || isExpired) {
			return
		}

		quizUiStore.upsertAnswer({
			questionId,
			answerIndex,
			certaintyLevel: certaintyLevel ?? null
		})

		saveInFlightByQuestionId = {
			...saveInFlightByQuestionId,
			[questionId]: true
		}

		const { error } = await attemptService.saveAnswer(activeAttempt.attemptId, questionId, {
			answerIndex,
			certaintyLevel
		})

		saveInFlightByQuestionId = {
			...saveInFlightByQuestionId,
			[questionId]: false
		}

		if (error) {
			toast.error(toUserMessage(error))
		}
	}

	const handleFinish = async () => {
		if (!activeAttempt) {
			return
		}

		const { value, error } = await submitMutation.mutateAsync(activeAttempt.attemptId)

		if (error) {
			toast.error(toUserMessage(error))
			return
		}

		toast.success(`Intento entregado (${value.answers.length}/${totalQuestions}).`)
		quizUiStore.leaveQuizAttempt()
	}

	$effect(() => {
		if (!isExpired || autoSubmitStarted || !activeAttempt) {
			return
		}

		autoSubmitStarted = true
		void handleFinish()
	})
</script>

{#if activeAttempt && activeQuiz && currentQuestion}
	<section class="panel-surface flex h-full min-h-0 flex-col gap-4 p-4 sm:p-5">
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="m-0 text-xl text-black">{activeQuiz.title}</h3>
				<p class="mt-1 text-sm leading-relaxed text-zinc-700">
					Pregunta {quizUiStore.currentQuestionIndex + 1} de {totalQuestions} - Respondidas:
					{answeredCount}
				</p>
				<div class="mt-3 w-full max-w-xs">
					<div class="h-2 overflow-hidden rounded-full bg-zinc-200">
						<div class="h-full rounded-full bg-black transition-[width] duration-200" style={`width: ${progress}%`}></div>
					</div>
					<p class="mt-1 text-xs text-zinc-600">{progress}% completado</p>
				</div>
			</div>
			<div class="flex flex-wrap items-center gap-2">
				<span class="code-chip">{remainingLabel}</span>
				<button class="btn-secondary" type="button" onclick={() => quizUiStore.leaveQuizAttempt()}>
					Salir
				</button>
			</div>
		</div>

		<article class="panel-muted min-h-0 flex-1 overflow-auto p-4 sm:p-5">
			<p class="m-0 text-lg leading-relaxed text-black sm:text-xl">{currentQuestion.question}</p>

			{#if currentQuestion.images.length > 0}
				<div class="mt-4 grid gap-3 sm:grid-cols-2">
					{#each currentQuestion.images as imageUrl}
						<img class="w-full rounded-[4px] border border-zinc-300 bg-white" src={imageUrl} alt="Imagen de apoyo" />
					{/each}
				</div>
			{/if}

			<div class="mt-5 grid gap-2.5">
				{#each currentQuestion.options as option, optionIndex}
					<button
						class={`rounded-[4px] border px-4 py-3 text-left text-base leading-relaxed transition ${
							selectedAnswer === optionIndex
								? 'border-black bg-black text-white shadow-[0_10px_20px_rgba(0,0,0,0.08)]'
								: 'border-zinc-300 bg-white text-black hover:border-zinc-400 hover:bg-zinc-50'
						}`}
						type="button"
						onclick={() => saveAnswer(currentQuestion.questionId, optionIndex)}
						disabled={isExpired || submitMutation.isPending}
					>
						{option}
					</button>
				{/each}
			</div>
		</article>

		<div class="flex flex-wrap items-center justify-between gap-3">
			<p class="m-0 text-sm text-zinc-600">
				{#if isExpired}
					Tiempo agotado. Intentando entregar automaticamente...
				{:else if selectedAnswer === undefined}
					Selecciona una alternativa para continuar.
				{:else if saveInFlightByQuestionId[currentQuestion.questionId]}
					Guardando respuesta...
				{:else}
					Respuesta guardada.
				{/if}
			</p>
			<div class="flex flex-wrap items-center gap-2">
				{#if !isLastQuestion}
					<button
						class="btn-secondary"
						type="button"
						onclick={() => quizUiStore.goToQuestion(quizUiStore.currentQuestionIndex + 1)}
						disabled={saveInFlightByQuestionId[currentQuestion.questionId] || selectedAnswer === undefined}
					>
						Siguiente
						<ChevronRight size={14} class="ml-1 inline" />
					</button>
				{/if}
				<button class="btn-primary" type="button" onclick={handleFinish} disabled={submitMutation.isPending || isExpired}>
					<Send size={14} class="mr-1 inline" />
					{submitMutation.isPending ? 'Entregando...' : isLastQuestion ? 'Finalizar intento' : 'Entregar ahora'}
				</button>
			</div>
		</div>

		<p class="m-0 flex items-center gap-1 text-xs text-zinc-600">
			<ClipboardCheck size={13} />
			Las respuestas se guardan por questionId y el tiempo restante se calcula desde expiresAt.
		</p>
	</section>
{/if}
