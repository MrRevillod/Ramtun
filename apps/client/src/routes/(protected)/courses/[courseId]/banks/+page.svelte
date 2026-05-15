<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import * as v from "valibot"
	import { toast } from "svelte-sonner"
	import { fade, scale } from "svelte/transition"
	import { Upload, Code, Trash2, Plus, X } from "lucide-svelte"
	import { banksService } from "$lib/banks/banks.service"
	import { coursesService } from "$lib/courses/courses.service"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"
	import CodeBlock from "$lib/shared/components/CodeBlock.svelte"
	import { getErrorMessage } from "$lib/shared/errors"
	import {
		bankUploadSchema,
		bankQuestionsSchema,
		type QuestionInput,
	} from "$lib/banks/banks.schema"

	let { data } = $props()

	const queryClient = useQueryClient()

	const courseQuery = createQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.getOrThrow(data.courseId),
	}))

	const banksQuery = createQuery(() => ({
		queryKey: ["banks", data.courseId],
		queryFn: () => banksService.listByCourseOrThrow(data.courseId),
	}))

	let bankName = $state("")
	let selectedFile = $state<File | null>(null)
	let showCreateModal = $state(false)
	let showFormatModal = $state(false)
	let fileInput = $state<HTMLInputElement | null>(null)
	let bankToDelete = $state<{ id: string; name: string } | null>(null)

	const newQuestionSchema = v.object({
		prompt: v.string(),
		options: v.array(v.string()),
		answerIndex: v.number(),
		images: v.optional(v.array(v.string())),
	})

	const legacyQuestionSchema = v.object({
		question: v.string(),
		options: v.array(v.string()),
		answer: v.number(),
		images: v.optional(v.array(v.string())),
	})

	const code1 = `[
 	{
	    "prompt": "¿Cuál es la capital de Francia?",
	    "options": ["París", "Londres", "Berlín"],
	    "answerIndex": 0,
	    "images": []
	},
  	{
	    "prompt": "¿Cuánto es 2 + 2?",
	    "options": ["3", "4", "5", "6"],
	    "answerIndex": 1,
	    "images": []
    }
]`

	const code2 = `{
  "questions": [
    {
      "question": "¿Cuál es la capital de Francia?",
      "options": ["París", "Londres", "Berlín"],
      "answer": 0,
      "images": []
    },
    {
      "question": "¿Cuánto es 2 + 2?",
      "options": ["3", "4", "5", "6"],
      "answer": 1,
      "images": []
    }
  ]
}`

	const legacyBankSchema = v.object({
		questions: v.array(legacyQuestionSchema),
	})

	type LegacyQuestion = v.InferInput<typeof legacyQuestionSchema>
	type NewQuestion = v.InferInput<typeof newQuestionSchema>

	const normalizeQuestions = (parsed: unknown): QuestionInput[] => {
		const newResult = v.safeParse(v.array(newQuestionSchema), parsed)
		if (newResult.success) {
			return newResult.output.map((q: NewQuestion) => ({
				prompt: q.prompt,
				options: q.options,
				answerIndex: q.answerIndex,
				images: q.images ?? [],
			}))
		}

		const legacyResult = v.safeParse(legacyBankSchema, parsed)
		if (legacyResult.success) {
			return legacyResult.output.questions.map((q: LegacyQuestion) => ({
				prompt: q.question,
				options: q.options,
				answerIndex: q.answer,
				images: q.images ?? [],
			}))
		}

		throw new Error(
			"El JSON debe ser un array de preguntas o un objeto con 'questions'."
		)
	}

	const uploadBankMutation = createMutation(() => ({
		mutationFn: async () => {
			if (!selectedFile) {
				throw new Error("Selecciona un archivo JSON.")
			}

			const text = await selectedFile.text()
			let parsed: unknown

			try {
				parsed = JSON.parse(text)
			} catch {
				throw new Error("El archivo no contiene JSON válido.")
			}

			const rawQuestions = normalizeQuestions(parsed)

			const mapped = rawQuestions.map(q => ({
				...q,
				images: q.images ?? [],
			}))

			const questionsResult = v.safeParse(bankQuestionsSchema, mapped)

			if (!questionsResult.success) {
				const firstIssue = questionsResult.issues[0]
				throw new Error(firstIssue?.message ?? "Estructura de preguntas inválida.")
			}

			const questions = questionsResult.output as QuestionInput[]

			const nameResult = v.safeParse(
				v.pipe(v.string(), v.trim(), v.minLength(1), v.maxLength(120)),
				bankName
			)

			if (!nameResult.success) {
				throw new Error("El nombre del banco debe tener entre 1 y 120 caracteres.")
			}

			const payload = {
				courseId: data.courseId,
				name: nameResult.output,
				questions,
			}

			const fullResult = v.safeParse(bankUploadSchema, payload)
			if (!fullResult.success) {
				const firstIssue = fullResult.issues[0]
				throw new Error(firstIssue?.message ?? "Datos inválidos.")
			}

			await banksService.createOrThrow(
				payload as unknown as import("$lib/banks/types").CreateQuestionBankInput
			)
		},
		onSuccess: async () => {
			toast.success("Banco creado correctamente.")
			bankName = ""
			selectedFile = null
			showCreateModal = false
			if (fileInput) fileInput.value = ""
			await queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const deleteBankMutation = createMutation(() => ({
		mutationFn: (bankId: string) => banksService.softDeleteOrThrow(bankId),
		onSuccess: async () => {
			toast.success("Banco eliminado correctamente.")
			await queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const handleFileChange = (event: Event) => {
		const target = event.target as HTMLInputElement
		const file = target.files?.[0]
		if (file) {
			selectedFile = file
		}
	}

	const handleSubmit = async () => {
		await uploadBankMutation.mutateAsync()
	}

	const formatCreatedAt = (iso: string) => {
		const parsed = new Date(iso)
		if (Number.isNaN(parsed.getTime())) {
			console.warn("[banks] invalid createdAt", { iso })
			return "Fecha no disponible"
		}

		return new Intl.DateTimeFormat("es-CL", {
			dateStyle: "medium",
			timeStyle: "short",
		}).format(parsed)
	}

	const confirmDeleteBank = async () => {
		if (!bankToDelete) return
		await deleteBankMutation.mutateAsync(bankToDelete.id)
		bankToDelete = null
	}
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="mt-2 mb-0 text-xl text-black">
					{courseQuery.data?.name ?? "Curso"} - Bancos de preguntas
				</h3>
				<p class="m-0 mt-2 text-zinc-700">
					Carga y organiza preguntas para reutilizarlas al crear quizzes.
				</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showCreateModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Nuevo banco
			</button>
		</div>
	</header>

	<section class="panel-elevated p-4">
		{#if banksQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando bancos...</p>
		{:else if banksQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(banksQuery.error)}</p>
		{:else if !banksQuery.data?.length}
			<p class="m-0 text-zinc-600">Aún no existen bancos para este curso.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Preguntas</th>
							<th class="px-3 py-2 text-left font-medium">Creado</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each banksQuery.data as bank (bank.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() =>
									goto(resolve(`/courses/${data.courseId}/banks/${bank.id}`))}
							>
								<td class="px-3 py-2 text-zinc-900">{bank.name}</td>
								<td class="px-3 py-2 text-zinc-700">{bank.questions.length}</td>
								<td class="px-3 py-2 text-zinc-700"
									>{formatCreatedAt(bank.created_at ?? bank.createdAt ?? "")}</td
								>
								<td class="px-3 py-2">
									<button
										class="icon-btn icon-btn-danger"
										title="Eliminar"
										type="button"
										onclick={e => {
											e.stopPropagation()
											bankToDelete = { id: bank.id, name: bank.name }
										}}
										disabled={deleteBankMutation.isPending}
									>
										<Trash2 size={15} aria-hidden="true" />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>

	{#if showCreateModal}
		<div
			class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			transition:fade={{ duration: 180 }}
			onclick={() => (showCreateModal = false)}
			onkeydown={e => {
				if (e.key === "Escape") showCreateModal = false
			}}
		>
			<section
				class="panel-elevated w-full max-w-2xl p-5"
				role="presentation"
				tabindex="-1"
				transition:scale={{ duration: 190, start: 0.98 }}
				onclick={e => e.stopPropagation()}
			>
				<div class="mb-3 flex items-center justify-between gap-2">
					<h4 class="m-0 text-base text-black">Subir banco de preguntas</h4>
					<button
						class="btn-tertiary p-1"
						type="button"
						onclick={() => (showCreateModal = false)}
						><X size={18} aria-hidden="true" /></button
					>
				</div>
				<section class="page-main">
					<div class="form-stack">
						<h4 class="mt-2 mb-0 text-base text-black">Subir banco de preguntas</h4>
						<p class="mt-1 mb-0 text-sm text-zinc-600">
							Selecciona un archivo JSON validado.
						</p>

						<div class="form-stack mt-3">
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Nombre del banco</span>
								<input
									class="input-base"
									type="text"
									bind:value={bankName}
									placeholder="Ej: Guía 1 - Mecánica"
								/>
							</label>

							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Archivo JSON</span>
								<input
									class="input-base file:cursor-pointer"
									type="file"
									accept=".json,application/json"
									onchange={handleFileChange}
									bind:this={fileInput}
								/>
								{#if selectedFile}
									<span class="text-xs text-zinc-600">{selectedFile.name}</span>
								{/if}
							</label>

							<div class="sticky-actions">
								<button
									class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
									type="button"
									onclick={handleSubmit}
									disabled={uploadBankMutation.isPending ||
										!bankName ||
										!selectedFile}
								>
									<Upload size={16} aria-hidden="true" />
									{uploadBankMutation.isPending ? "Subiendo..." : "Subir banco"}
								</button>
								<button
									class="btn-secondary flex w-full items-center gap-1.5 sm:w-auto"
									type="button"
									onclick={() => (showFormatModal = true)}
								>
									<Code size={16} aria-hidden="true" />
									Ver formato JSON
								</button>
							</div>
						</div>
					</div>
				</section>
			</section>
		</div>
	{/if}

	{#if showFormatModal}
		<div
			class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			onclick={() => (showFormatModal = false)}
			onkeydown={e => {
				if (e.key === "Escape") showFormatModal = false
			}}
		>
			<div
				class="w-full max-w-2xl rounded-lg bg-white p-6 shadow-xl"
				role="presentation"
				tabindex="-1"
				onclick={e => {
					e.stopPropagation()
				}}
			>
				<h4 class="m-0 text-lg text-black">Formato JSON esperado</h4>
				<p class="mt-2 mb-4 text-sm text-zinc-600">
					Se aceptan dos formatos. El primero es el recomendado.
				</p>

				<p class="mt-2 mb-1 text-sm font-semibold text-zinc-800">
					Formato actual (recomendado)
				</p>
				<CodeBlock code={code1} class="mt-2" />

				<ul class="mt-4 mb-0 list-inside list-disc text-sm text-zinc-700">
					<li>
						<code class="text-zinc-900">prompt</code>: texto de la pregunta (1-1000
						caracteres)
					</li>
					<li>
						<code class="text-zinc-900">options</code>: array de 2 a 5 opciones
					</li>
					<li>
						<code class="text-zinc-900">answerIndex</code>: índice de la respuesta
						correcta (0-based)
					</li>
					<li>
						<code class="text-zinc-900">images</code>: array de URLs de imágenes
						(opcional, máx 5)
					</li>
				</ul>

				<p class="mt-4 mb-1 text-sm font-semibold text-zinc-800">
					Formato legado (compatible)
				</p>
				<CodeBlock code={code2} class="mt-4" />

				<p class="mt-2 mb-0 text-sm text-zinc-600">
					Usa <code class="text-zinc-900">question</code> y
					<code class="text-zinc-900">answer</code> en lugar de
					<code class="text-zinc-900">prompt</code> y
					<code class="text-zinc-900">answerIndex</code>.
				</p>

				<div class="mt-5 flex justify-end">
					<button
						class="btn-primary"
						type="button"
						onclick={() => (showFormatModal = false)}
					>
						Cerrar
					</button>
				</div>
			</div>
		</div>
	{/if}

	<ConfirmActionModal
		open={!!bankToDelete}
		title="Eliminar banco"
		message={bankToDelete ? `Se eliminara el banco ${bankToDelete.name}.` : ""}
		confirmLabel="Eliminar"
		isPending={deleteBankMutation.isPending}
		onCancel={() => (bankToDelete = null)}
		onConfirm={confirmDeleteBank}
	/>
</section>
