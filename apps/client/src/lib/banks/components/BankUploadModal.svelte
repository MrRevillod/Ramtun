<script lang="ts">
	import * as v from "valibot"
	import type { CreateMutationResult } from "@tanstack/svelte-query"
	import type { CreateQuestionBankInput } from "$lib/banks/banks.dtos"
	import type { SubmitEventHandler } from "@formisch/svelte"
	import type { QuestionInput } from "$lib/banks/banks.dtos"

	import { scale } from "svelte/transition"
	import { toast } from "svelte-sonner"
	import { Upload, Code, X } from "lucide-svelte"
	import { bankQuestionsSchema } from "$lib/banks/banks.dtos"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { inlineTryAsync } from "$lib/shared/try"

	import CodeBlock from "$lib/shared/components/CodeBlock.svelte"

	interface BankUploadModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: () => void
		mutation: CreateMutationResult<void, Error, CreateQuestionBankInput, unknown>
	}

	let { open, courseId, onclose, onsuccess, mutation }: BankUploadModalProps = $props()

	const bankUploadFormSchema = v.object({
		name: v.pipe(
			v.string(),
			v.trim(),
			v.minLength(1, "El nombre es obligatorio."),
			v.maxLength(120, "Máximo 120 caracteres.")
		),
	})

	const form = createForm({
		schema: bankUploadFormSchema,
		initialInput: { name: "" },
	})

	let selectedFile = $state<File | null>(null)
	let parsedQuestions = $state<QuestionInput[]>([])
	let fileError = $state<string | null>(null)
	let fileInput = $state<HTMLInputElement | null>(null)
	let showFormatModal = $state(false)

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

	const handleFileChange = async (e: Event) => {
		const file = (e.target as HTMLInputElement).files?.[0] ?? null
		selectedFile = file
		fileError = null
		parsedQuestions = []

		if (!file) return

		const [parsed, parseError] = await inlineTryAsync<unknown>(
			() => file.text().then(t => JSON.parse(t))
		)

		if (parseError !== null) {
			fileError = "El archivo no contiene un JSON válido."
			return
		}

		const result = v.safeParse(bankQuestionsSchema, parsed)
		if (!result.success) {
			fileError = result.issues[0]?.message ?? "El archivo JSON no es válido."
			return
		}

		parsedQuestions = result.output
	}

	const handleSubmit: SubmitEventHandler<typeof bankUploadFormSchema> = async output => {
		if (!selectedFile) {
			toast.error("Selecciona un archivo JSON.")
			return
		}

		if (fileError) {
			toast.error("Corrige los errores del archivo antes de continuar.")
			return
		}

		await mutation.mutateAsync({
			courseId,
			name: output.name,
			questions: parsedQuestions,
		})

		reset(form)
		selectedFile = null
		parsedQuestions = []
		fileError = null
		if (fileInput) fileInput.value = ""

		onsuccess()
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={onclose}
		onkeydown={e => {
			if (e.key === "Escape") onclose()
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
				<button class="btn-tertiary p-1" type="button" onclick={onclose}>
					<X size={18} aria-hidden="true" />
				</button>
			</div>
			<section class="page-main">
				<div class="form-stack">
					<div class="form-stack mt-3">
						<Form of={form} onsubmit={handleSubmit} class="form-stack">
							<Field of={form} path={["name"]}>
								{#snippet children(field)}
									<label class="grid gap-1.5">
										<span class="text-sm text-zinc-800">Nombre del banco</span>
										<input
											{...field.props}
											class="input-base"
											value={field.input ?? ""}
											placeholder="Ej: Guía 1 - Mecánica"
										/>
										{#if field.errors?.[0]}
											<span class="text-sm text-red-700">{field.errors[0]}</span>
										{/if}
									</label>
								{/snippet}
							</Field>

							<div class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Archivo JSON</span>
								<label
									class="btn-secondary file-label flex cursor-pointer items-center justify-start gap-1.5 text-left"
								>
									<Upload size={16} aria-hidden="true" />
									{selectedFile ? selectedFile.name : "Seleccionar archivo JSON"}
									<input
										type="file"
										accept=".json,application/json"
										class="sr-only"
										onchange={handleFileChange}
										bind:this={fileInput}
									/>
								</label>
								{#if fileError}
									<span class="text-sm text-red-700">{fileError}</span>
								{/if}
							</div>

							<div class="sticky-actions mt-">
								<button
									class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
									type="submit"
									disabled={mutation.isPending || !selectedFile || !!fileError}
								>
									<Upload size={16} aria-hidden="true" />
									{mutation.isPending ? "Subiendo..." : "Subir banco"}
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
						</Form>
					</div>
				</div>
			</section>
		</section>
	</div>
{/if}

{#if showFormatModal}
	<div
		class="fixed inset-0 z-60 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		onclick={() => (showFormatModal = false)}
		onkeydown={e => {
			if (e.key === "Escape") showFormatModal = false
		}}
	>
		<div
			class="w-full max-w-2xl rounded-lg bg-white p-6"
			role="presentation"
			tabindex="-1"
			onclick={e => e.stopPropagation()}
		>
			<h4 class="m-0 text-lg text-black">Formato JSON esperado</h4>
			<p class="mt-2 mb-4 text-sm text-zinc-600">
				El archivo debe ser un JSON con un array de preguntas.
			</p>

			<p class="mt-2 mb-1 text-sm font-semibold text-zinc-800">Formato</p>
			<CodeBlock code={code1} class="mt-2" />

			<ul class="mt-4 mb-0 list-inside list-disc text-sm text-zinc-700">
				<li>
					<code class="text-zinc-900">prompt</code>: texto de la pregunta (1-1000 caracteres)
				</li>
				<li>
					<code class="text-zinc-900">options</code>: array de 2 a 5 opciones
				</li>
				<li>
					<code class="text-zinc-900">answerIndex</code>: índice de la respuesta correcta (0-based)
				</li>
				<li>
					<code class="text-zinc-900">images</code>: array de URLs de imágenes (opcional, máx 5)
				</li>
			</ul>

			<div class="mt-5 flex justify-end">
				<button class="btn-primary" type="button" onclick={() => (showFormatModal = false)}>
					Cerrar
				</button>
			</div>
		</div>
	</div>
{/if}
