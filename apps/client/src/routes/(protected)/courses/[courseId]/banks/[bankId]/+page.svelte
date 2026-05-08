<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { ArrowLeft } from "lucide-svelte"
	import { banksService } from "$lib/banks/banks.service"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"

	let { data } = $props()

	const courseQuery = createQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.getOrThrow(data.courseId),
	}))

	const bankQuery = createQuery(() => ({
		queryKey: ["bank", data.bankId],
		queryFn: () => banksService.getByIdOrThrow(data.bankId),
	}))
</script>

<section class="grid gap-4">
	<header class="flex flex-wrap items-start justify-between gap-3">
		<div>
			<h3 class="mt-2 mb-0 text-xl text-black">
				{courseQuery.data?.name ?? "Curso"} - Vista previa del banco
			</h3>
			<p class="m-0 mt-2 text-zinc-700">Revisa preguntas y alternativas del banco seleccionado.</p>
		</div>
		<a class="btn-secondary flex items-center gap-1.5" href={`/courses/${data.courseId}/banks`}>
			<ArrowLeft size={16} aria-hidden="true" />
			Volver a Bancos de preguntas
		</a>
	</header>

	<section class="panel-elevated p-4">
		{#if bankQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando banco...</p>
		{:else if bankQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(bankQuery.error)}</p>
		{:else if bankQuery.data}
			<div class="grid gap-4">
				<div class="rounded-md border border-zinc-200 bg-zinc-50 p-3">
					<p class="m-0 text-sm text-zinc-800"><strong>Banco:</strong> {bankQuery.data.name}</p>
					<p class="m-0 mt-1 text-sm text-zinc-700">
						<strong>Total de preguntas:</strong> {bankQuery.data.questions.length}
					</p>
				</div>

				<div class="grid gap-3">
					{#each bankQuery.data.questions as question, index (question.id)}
						<article class="rounded-md border border-zinc-200 bg-white/80 p-3">
							<p class="m-0 font-medium text-zinc-900">{index + 1}. {question.prompt}</p>
							<ul class="mt-2 grid gap-1 pl-4 text-sm text-zinc-700">
								{#each question.options as option, optionIndex}
									<li class={optionIndex === (question.answer_index ?? question.answerIndex)
										? "font-medium text-zinc-900"
										: ""}>
										{option}
										{#if optionIndex === (question.answer_index ?? question.answerIndex)}
											<span class="ml-2 text-xs text-zinc-500">(Correcta)</span>
										{/if}
									</li>
								{/each}
							</ul>
						</article>
					{/each}
				</div>
			</div>
		{/if}
	</section>
</section>
