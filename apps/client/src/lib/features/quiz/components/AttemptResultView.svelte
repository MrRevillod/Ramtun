<script lang="ts">
	import { BarChart3, CheckCircle2, CircleX, Trophy } from "lucide-svelte"
	import QuestionRichText from "$lib/features/quiz/components/QuestionRichText.svelte"
	import { quizUiStore } from "$lib/features/quiz/quiz.store.svelte"
	import type {
		AttemptCertaintyLevel,
		AttemptResult,
	} from "$lib/features/quiz/types"

	const result = $derived(quizUiStore.currentAttemptResult as AttemptResult | null)

	const gradeLabel = $derived.by(() => {
		if (!result) return "1,0"

		return new Intl.NumberFormat("es-CL", {
			minimumFractionDigits: 1,
			maximumFractionDigits: 2,
		}).format(result.grade)
	})

	const pointsLabel = $derived.by(() => {
		if (!result) return "0 / 0"

		const formatter = new Intl.NumberFormat("es-CL", {
			minimumFractionDigits: 0,
			maximumFractionDigits: 2,
		})

		return `${formatter.format(result.scorePoints)} / ${formatter.format(result.scorePointsMax)}`
	})

	const formatDate = (value: string) =>
		new Intl.DateTimeFormat("es-CL", {
			dateStyle: "medium",
			timeStyle: "short",
		}).format(new Date(value))

	const formatPoints = (value: number) =>
		new Intl.NumberFormat("es-CL", {
			minimumFractionDigits: 0,
			maximumFractionDigits: 2,
		}).format(value)

	const CERTAINTY_LEVELS: {
		level: AttemptCertaintyLevel
		label: string
	}[] = [
		{ level: "low", label: "Baja" },
		{ level: "medium", label: "Media" },
		{ level: "high", label: "Alta" },
	]

	const formatSignedPoints = (value: number) => {
		const formatted = formatPoints(value)

		if (value > 0) {
			return `+${formatted}`
		}

		if (value < 0) {
			return `-${formatPoints(Math.abs(value))}`
		}

		return formatted
	}

	const getOptionClass = (
		optionIndex: number,
		correctAnswerIndex: number,
		answerIndex: number | null
	) => {
		if (optionIndex === correctAnswerIndex) {
			return "border-emerald-700 bg-emerald-50 text-emerald-900"
		}

		if (
			answerIndex !== null &&
			optionIndex === answerIndex &&
			answerIndex !== correctAnswerIndex
		) {
			return "border-red-700 bg-red-50 text-red-900"
		}

		if (answerIndex !== null && optionIndex === answerIndex) {
			return "border-black bg-black text-white"
		}

		return "border-zinc-300 bg-white text-black"
	}
</script>

{#if result}
	<section class="panel-surface flex h-full min-h-0 flex-col gap-5 p-4 sm:p-5">
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<p class="section-kicker m-0">Resultado del intento</p>
				<h3 class="mt-1 mb-0 text-2xl text-black">Correccion final</h3>
				<p class="mt-2 text-sm text-zinc-700">
					Enviado: {formatDate(result.submittedAt)} - Evaluado: {formatDate(
						result.evaluatedAt
					)}
				</p>
			</div>
			<button
				class="btn-secondary"
				type="button"
				onclick={() => quizUiStore.closeAttemptResult()}
			>
				Volver al panel
			</button>
		</div>

		<div class="grid gap-3 sm:grid-cols-2">
			<div class="panel-muted p-4">
				<p class="m-0 flex items-center gap-2 text-sm font-medium text-zinc-700">
					<Trophy size={15} /> Nota final
				</p>
				<p class="mt-2 mb-0 text-3xl font-semibold text-black">{gradeLabel}</p>
			</div>
			<div class="panel-muted p-4">
				<p class="m-0 flex items-center gap-2 text-sm font-medium text-zinc-700">
					<BarChart3 size={15} /> Puntaje
				</p>
				<p class="mt-2 mb-0 text-3xl font-semibold text-black">{pointsLabel}</p>
			</div>
		</div>

		<div class="min-h-0 space-y-4 overflow-auto pr-1">
			{#each result.questions as question, index (question.questionId)}
				<article class="panel-muted p-4 sm:p-5">
					<div class="flex flex-wrap items-start justify-between gap-2">
						<p class="m-0 text-sm font-medium text-zinc-700">Pregunta {index + 1}</p>
						{#if question.isCorrect}
							<span class="inline-flex items-center gap-1 text-sm text-emerald-700">
								<CheckCircle2 size={14} /> Correcta
							</span>
						{:else}
							<span class="inline-flex items-center gap-1 text-sm text-red-700">
								<CircleX size={14} /> Incorrecta
							</span>
						{/if}
					</div>

					<div class="mt-3 text-base text-black sm:text-lg">
						<QuestionRichText text={question.question} />
					</div>

					{#if question.images.length > 0}
						<div class="mt-4 grid gap-3 sm:grid-cols-2">
							{#each question.images as imageUrl (imageUrl)}
								<img
									class="w-full rounded-sm border border-zinc-300 bg-white"
									src={imageUrl}
									alt="Imagen de apoyo"
								/>
							{/each}
						</div>
					{/if}

					<div class="mt-4 grid gap-2.5">
						{#each question.options as option, optionIndex (`${question.questionId}-${optionIndex}`)}
							{@const isSelected =
								question.answerIndex !== null &&
								optionIndex === question.answerIndex}
							<div
								class={`rounded-sm border px-4 py-3 text-left text-base leading-relaxed ${getOptionClass(optionIndex, question.correctAnswerIndex, question.answerIndex)}`}
							>
								<div class="flex items-center justify-between gap-3">
									<div class="min-w-0 flex-1">
										<QuestionRichText text={option} />
									</div>
									{#if isSelected}
										<span
											class="inline-flex h-6 min-w-12 shrink-0 items-center justify-center rounded-sm border border-zinc-300 bg-white/90 px-2 text-xs font-semibold text-zinc-700"
										>
											{formatSignedPoints(question.awardedPoints)}pt
										</span>
										<span
											class="inline-flex h-6 min-w-12 shrink-0 items-center justify-center rounded-sm border border-blue-500 bg-blue-50 px-2 text-xs font-semibold text-blue-800"
										>
											Tu selección
										</span>
									{/if}
								</div>
							</div>
						{/each}
					</div>

					{#if question.certaintyLevel}
						<div class="mt-4 grid gap-2 border-t border-zinc-200 pt-4">
							<p class="m-0 text-sm font-medium text-black">
								Nivel de certeza marcado
							</p>
							<div class="grid gap-2 sm:grid-cols-3">
								{#each CERTAINTY_LEVELS as item (item.level)}
									<div
										class={`rounded-sm border px-4 py-3 text-left transition ${
											question.certaintyLevel === item.level
												? "border-black bg-black text-white shadow-[0_10px_20px_rgba(0,0,0,0.08)]"
												: "border-zinc-300 bg-white text-black"
										}`}
									>
										<span class="block text-sm font-medium">{item.label}</span>
									</div>
								{/each}
							</div>
						</div>
					{/if}

					{#if question.answerIndex === null}
						<p class="mt-4 mb-0 text-sm text-zinc-700">
							Sin respuesta en esta pregunta.
						</p>
					{/if}
				</article>
			{/each}
		</div>
	</section>
{/if}
