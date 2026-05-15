<script lang="ts">
	import { CheckCircle2, XCircle } from "lucide-svelte"
	import type { AttemptResult } from "$lib/attempts/types"
	import { GradeValue } from "$lib/shared/value-objects/grade.value"
	import { certaintyLevelLabel } from "$lib/shared/labels"

	let { result }: { result: AttemptResult } = $props()

	const correctnessLabel = (isCorrect: boolean) =>
		isCorrect ? "Correcta" : "Incorrecta"
</script>

<section class="panel-surface p-4 sm:p-5">
	<div class="mb-3">
		<p class="m-0 text-xs text-zinc-600">Estudiante</p>
		<p class="m-0 mt-1 text-sm font-semibold text-zinc-800">{result.userName}</p>
	</div>

	<div class="mb-4 grid gap-2 sm:grid-cols-3">
		<div class="panel-muted p-3">
			<p class="m-0 text-xs text-zinc-600">Puntaje</p>
			<p class="m-0 mt-1 text-sm font-semibold text-zinc-800">
				{result.score} / {result.maxScore}
			</p>
		</div>
		<div class="panel-muted p-3">
			<p class="m-0 text-xs text-zinc-600">Nota</p>
			<p class="m-0 mt-1 text-sm font-semibold text-zinc-800">
				{GradeValue.format(result.grade)}
			</p>
		</div>
		<div class="panel-muted p-3">
			<p class="m-0 text-xs text-zinc-600">Enviado</p>
			<p class="m-0 mt-1 text-sm text-zinc-800">
				{new Date(result.submittedAt).toLocaleString()}
			</p>
		</div>
	</div>

	<div class="grid gap-3">
		{#each result.questions as question, index (question.questionId)}
			<article class="rounded-sm border border-zinc-200 bg-white p-3">
				<div class="mb-2 flex flex-wrap items-center justify-between gap-2">
					<p class="m-0 text-sm font-medium text-black">
						{index + 1}. {question.question}
					</p>
					<span
						class={question.isCorrect
							? "inline-flex items-center gap-1 rounded-sm bg-emerald-100 px-2 py-1 text-xs text-emerald-800"
							: "inline-flex items-center gap-1 rounded-sm bg-red-100 px-2 py-1 text-xs text-red-800"}
					>
						{#if question.isCorrect}
							<CheckCircle2 size={12} aria-hidden="true" />
						{:else}
							<XCircle size={12} aria-hidden="true" />
						{/if}
						{correctnessLabel(question.isCorrect)}
					</span>
				</div>

				<ul class="m-0 grid gap-1 p-0">
					{#each question.options as option, optionIndex (option)}
						<li
							class={optionIndex === question.correctAnswerIndex
								? "list-none rounded-sm border border-emerald-300 bg-emerald-50 px-2 py-1 text-sm text-emerald-900"
								: optionIndex === question.answerIndex
									? "list-none rounded-sm border border-red-300 bg-red-50 px-2 py-1 text-sm text-red-900"
									: "list-none rounded-sm border border-zinc-200 bg-zinc-50 px-2 py-1 text-sm text-zinc-700"}
						>
							{option}
						</li>
					{/each}
				</ul>

				<p class="mt-2 mb-0 text-xs text-zinc-600">
					Puntos: {question.awardedPoints} · Certeza:
					{question.certaintyLevel
						? certaintyLevelLabel(question.certaintyLevel)
						: "Sin definir"}
				</p>
			</article>
		{/each}
	</div>
</section>
