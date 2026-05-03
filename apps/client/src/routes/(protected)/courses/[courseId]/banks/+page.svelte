<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import * as v from "valibot"
	import { toast } from "svelte-sonner"
	import { Upload, Code, RefreshCw, Trash2 } from "lucide-svelte"
	import { banksService } from "$lib/banks/banks.service"
	import { coursesService } from "$lib/courses/courses.service"
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
	let showFormatModal = $state(false)
	let fileInput = $state<HTMLInputElement | null>(null)

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
</script>

<section class="grid gap-4">
	<header>
		<h3 class="mt-2 mb-0 text-xl text-black">
			Bancos de preguntas {#if courseQuery.data}<span class="text-zinc-500"
					>· {courseQuery.data.code}</span
				>{/if}
		</h3>
		<p class="m-0 mt-2 text-zinc-700">
			Carga preguntas masivamente desde un archivo JSON.
		</p>
	</header>

	<section class="panel-muted p-4">
		<h4 class="m-0 text-base text-black">Subir banco de preguntas</h4>
		<p class="mt-1 mb-0 text-sm text-zinc-600">
			Selecciona un archivo JSON con la estructura requerida.
		</p>

		<div class="mt-3 grid gap-3">
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

			<div class="flex flex-wrap gap-2">
				<button
					class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
					type="button"
					onclick={handleSubmit}
					disabled={uploadBankMutation.isPending || !bankName || !selectedFile}
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
	</section>

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
				<pre
					class="overflow-x-auto rounded-md bg-zinc-50 p-4 text-sm text-zinc-800">{@html `<code>[
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
]</code>`}</pre>

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
				<pre
					class="overflow-x-auto rounded-md bg-zinc-50 p-4 text-sm text-zinc-800">{@html `<code>{
  "questions": [
    {
      "question": "¿Cuál es la capital de Francia?",
      "options": ["París", "Londres", "Berlín"],
      "answer": 0,
      "images": []
    }
  ]
}</code>`}</pre>

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

	<section class="panel-surface p-4">
		<div class="mb-3 flex items-center justify-between gap-3">
			<h4 class="m-0 text-base text-black">Bancos del curso</h4>
			<button
				class="btn-secondary flex items-center gap-1.5"
				type="button"
				onclick={() => banksQuery.refetch()}
				disabled={banksQuery.isFetching}
			>
				<RefreshCw size={16} aria-hidden="true" />
				Actualizar
			</button>
		</div>

		{#if banksQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando bancos...</p>
		{:else if banksQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(banksQuery.error)}</p>
		{:else if !banksQuery.data?.length}
			<p class="m-0 text-zinc-600">Aún no existen bancos para este curso.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="bg-zinc-100/90 text-zinc-700">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Preguntas</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each banksQuery.data as bank (bank.id)}
							<tr class="border-t border-zinc-200 bg-white/80">
								<td class="px-3 py-2 text-zinc-900">{bank.name}</td>
								<td class="px-3 py-2 text-zinc-700">{bank.questions.length}</td>
								<td class="px-3 py-2">
									<button
										class="btn-tertiary flex items-center gap-1.5"
										type="button"
										onclick={() => deleteBankMutation.mutate(bank.id)}
										disabled={deleteBankMutation.isPending}
									>
										<Trash2 size={16} aria-hidden="true" />
										Eliminar
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>
