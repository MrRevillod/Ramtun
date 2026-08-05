<script lang="ts">
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"
	import { ArrowLeft, Download, Pencil, Save, X } from "lucide-svelte"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { banksService } from "$lib/banks/banks.service"
	import { exportBankJson } from "$lib/banks/utils"
	import { coursesService } from "$lib/courses/service"
	import Markdown from "$lib/shared/components/Markdown.svelte"
	import MarkdownEditor from "$lib/shared/components/MarkdownEditor.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const courseQuery = useQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.findOne(data.courseId),
	}))

	const bankQuery = useQuery(() => ({
		queryKey: ["bank", data.bankId],
		queryFn: () => banksService.getById(data.bankId),
	}))

	type DraftQuestion = {
		prompt: string
		options: string[]
	}

	let editing = $state(false)
	let draft = $state<DraftQuestion[]>([])

	const correctIndex = (question: { answer_index?: number; answerIndex?: number }) =>
		question.answer_index ?? question.answerIndex

	const enterEdit = () => {
		const bank = bankQuery.data
		if (!bank) return

		draft = bank.questions.map((question) => ({
			prompt: question.prompt,
			options: question.options,
		}))
		editing = true
	}

	const cancelEdit = () => {
		editing = false
		draft = []
	}

	const updateMutation = useMutation(() => ({
		mutationFn: (questions: DraftQuestion[]) => banksService.update(data.bankId, { questions }),
		onSuccess: async () => {
			editing = false
			draft = []
			toast.success("Banco actualizado correctamente.")
			await queryClient.invalidateQueries({ queryKey: ["bank", data.bankId] })
			await queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const saveEdit = () => {
		if (updateMutation.isPending) return
		updateMutation.mutate(draft)
	}
</script>

<section class="grid gap-4">
	<header class="flex flex-wrap items-start justify-between gap-3">
		<div>
			<h3 class="mt-2 mb-0 text-xl text-black">
				{courseQuery.data?.name ?? "Curso"} - {bankQuery.data?.name ?? "Banco"}
			</h3>
			<p class="m-0 mt-2 text-sm text-zinc-700">
				{bankQuery.data?.questions.length ?? 0} preguntas
				{#if editing}
					- modo edición
				{/if}
			</p>
		</div>
		<div class="flex flex-wrap items-center gap-1.5">
			<a
				class="btn-secondary flex items-center gap-1.5"
				href={resolve(`/courses/${data.courseId}/banks`)}
			>
				<ArrowLeft size={16} aria-hidden="true" />
				Volver a Bancos de preguntas
			</a>

			{#if editing}
				<button
					class="btn-secondary flex items-center gap-1.5"
					type="button"
					onclick={cancelEdit}
					disabled={updateMutation.isPending}
				>
					<X size={16} aria-hidden="true" />
					Cancelar
				</button>
				<button
					class="btn-primary flex items-center gap-1.5"
					type="button"
					onclick={saveEdit}
					disabled={updateMutation.isPending}
				>
					<Save size={16} aria-hidden="true" />
					{updateMutation.isPending ? "Guardando..." : "Guardar"}
				</button>
			{:else if bankQuery.data}
				<button
					class="btn-secondary flex items-center gap-1.5"
					type="button"
					onclick={enterEdit}
				>
					<Pencil size={16} aria-hidden="true" />
					Editar
				</button>
				<button
					class="btn-secondary flex items-center gap-1.5"
					type="button"
					onclick={() => exportBankJson(bankQuery.data)}
				>
					<Download size={16} aria-hidden="true" />
					Descargar JSON
				</button>
			{/if}
		</div>
	</header>

	<section>
		{#if bankQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando banco...</p>
		{:else if bankQuery.error}
			<p class="m-0 text-red-700">
				{bankQuery.error?.messageOrDefault ?? ""}
			</p>
		{:else if bankQuery.data}
			{#if editing}
				<div class="grid gap-4">
					{#each draft as question, index (index)}
						<article class="panel-surface p-4 sm:p-5">
							<div class="mb-4 flex items-center justify-between gap-2">
								<h4 class="m-0 text-base font-medium text-black">
									Pregunta {index + 1}
								</h4>
							</div>

							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Enunciado (Markdown)</span>
								<MarkdownEditor bind:value={question.prompt} />
							</label>

							<div class="mt-4 grid gap-2">
								{#each question.options as _, optionIndex (optionIndex)}
									{@const correct = correctIndex(bankQuery.data.questions[index])}
									<div class="grid gap-1.5">
										<label class="grid gap-1.5">
											<span class="text-sm text-zinc-800">
												Opción {optionIndex + 1}
												{#if optionIndex === correct}
													<span class="text-emerald-700">
														(correcta)</span
													>
												{/if}
											</span>
											<input
												class="input-base"
												bind:value={question.options[optionIndex]}
											/>
										</label>
									</div>
								{/each}
							</div>
						</article>
					{/each}
				</div>
			{:else}
				<div class="grid gap-4">
					{#each bankQuery.data.questions as question, index (question.id)}
						<article class="panel-surface p-4 sm:p-5">
							<div class="mb-4 flex items-start gap-2">
								<span class="shrink-0 font-medium text-black">{index + 1}.</span>
								<div class="min-w-0 flex-1">
									<Markdown content={question.prompt} />
								</div>
							</div>
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
		{/if}
	</section>
</section>
