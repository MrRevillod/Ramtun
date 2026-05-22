<script lang="ts">
	import { Upload, Code, X } from "lucide-svelte"
	import * as v from "valibot"
	import { scale } from "svelte/transition"
	import type { QuestionInput } from "$lib/banks/banks.dtos"
	import { bankQuestionsSchema, normalizeQuestions } from "$lib/banks/banks.dtos"
	import CodeBlock from "$lib/shared/components/CodeBlock.svelte"
	import { toast } from "svelte-sonner"
	import { getErrorMessage } from "$lib/shared/errors"
	import type { CreateQuestionBankInput } from "$lib/banks/banks.dtos"
	import type { CreateMutationResult } from "@tanstack/svelte-query"

	interface BankUploadModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: () => void
		mutation: CreateMutationResult<void, Error, CreateQuestionBankInput, unknown>
	}

	let { open, courseId, onclose, onsuccess, mutation }: BankUploadModalProps =
		$props()

	let bankName = $state("")
	let selectedFile = $state<File | null>(null)
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

	const handleFileChange = (e: Event) => {
		const input = e.target as HTMLInputElement
		selectedFile = input.files?.[0] ?? null
	}

	const handleSubmit = async () => {
		try {
			if (!selectedFile) return

			const text = await selectedFile.text()
			let parsed: unknown

			try {
				parsed = JSON.parse(text)
			} catch {
				throw new Error("El archivo no contiene un JSON válido.")
			}

			const rawQuestions = normalizeQuestions(parsed)

			const mapped = rawQuestions.map(q => ({
				...q,
				images: q.images ?? [],
			}))

			const questionsResult = v.safeParse(bankQuestionsSchema, mapped)
			if (!questionsResult.success) {
				const issues = questionsResult.issues.map(i => i.message).join("; ")
				throw new Error(`Datos inválidos: ${issues}`)
			}

			const nameResult = v.safeParse(
				v.pipe(v.string(), v.trim(), v.minLength(1), v.maxLength(120)),
				bankName
			)
			if (!nameResult.success) {
				throw new Error("El nombre del banco es obligatorio.")
			}

			await mutation.mutateAsync({
				courseId,
				name: nameResult.output,
				questions: questionsResult.output as QuestionInput[],
			})

			onsuccess()
		} catch (err) {
			toast.error(getErrorMessage(err))
		}
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

						<div class="sticky-actions mt-">
							<button
								class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
								type="button"
								onclick={handleSubmit}
								disabled={mutation.isPending || !bankName || !selectedFile}
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
