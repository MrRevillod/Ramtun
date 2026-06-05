<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { ArrowLeft } from "lucide-svelte"
	import { resolve } from "$app/paths"
	import { banksService } from "$lib/banks/banks.service"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"

	let { data } = $props()

	const courseQuery = createQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.get(data.courseId),
	}))

	const bankQuery = createQuery(() => ({
		queryKey: ["bank", data.bankId],
		queryFn: () => banksService.getById(data.bankId),
	}))

	const correctIndex = (question: { answer_index?: number; answerIndex?: number }) =>
		question.answer_index ?? question.answerIndex
</script>

<section class="grid gap-4">
	<header class="flex flex-wrap items-start justify-between gap-3">
		<div>
			<h3 class="mt-2 mb-0 text-xl text-black">
				{courseQuery.data?.name ?? "Curso"} - {bankQuery.data?.name ?? "Banco"}
			</h3>
			<p class="m-0 mt-2 text-zinc-700">
				{bankQuery.data?.questions.length ?? 0} preguntas
			</p>
		</div>
		<a
			class="btn-secondary flex items-center gap-1.5"
			href={resolve(`/courses/${data.courseId}/banks`)}
		>
			<ArrowLeft size={16} aria-hidden="true" />
			Volver a Bancos de preguntas
		</a>
	</header>

	<section>
		{#if bankQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando banco...</p>
		{:else if bankQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(bankQuery.error)}</p>
		{:else if bankQuery.data}
			<div class="grid gap-4">
				{#each bankQuery.data.questions as question, index (question.id)}
					<article class="panel-surface p-4 sm:p-5">
						<h4 class="mt-0 mb-4 text-base leading-relaxed text-black">
							{index + 1}. {question.prompt}
						</h4>
						<div class="grid gap-2">
							{#each question.options as option, optionIndex (optionIndex)}
								<div
									class="quiz-option"
									data-active={optionIndex === correctIndex(question)}
								>
									{option}
								</div>
							{/each}
						</div>
					</article>
				{/each}
			</div>
		{/if}
	</section>
</section>
