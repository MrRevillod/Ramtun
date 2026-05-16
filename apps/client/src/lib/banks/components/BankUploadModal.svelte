<script lang="ts">
	import { Upload, Code, X } from "lucide-svelte"
	import * as v from "valibot"
	import { scale } from "svelte/transition"
	import type { QuestionInput } from "$lib/banks/banks.dtos"
	import { bankQuestionsSchema, normalizeQuestions } from "$lib/banks/banks.dtos"
	import CodeBlock from "$lib/shared/components/CodeBlock.svelte"

	interface BankUploadModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: () => void
		mutation: {
			isPending: boolean
			mutateAsync: (payload: {
				courseId: string
				name: string
				questions: QuestionInput[]
			}) => Promise<unknown>
		}
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

	const handleFileChange = (e: Event) => {
		const input = e.target as HTMLInputElement
		selectedFile = input.files?.[0] ?? null
	}

	const handleSubmit = async () => {
		if (!selectedFile) return

		const text = await selectedFile.text()
		let parsed: unknown

		try {
			parsed = JSON.parse(text)
		} catch {
			return
		}

		const rawQuestions = normalizeQuestions(parsed)

		const mapped = rawQuestions.map(q => ({
			...q,
			images: q.images ?? [],
		}))

		const questionsResult = v.safeParse(bankQuestionsSchema, mapped)
		if (!questionsResult.success) return

		const questions = questionsResult.output as QuestionInput[]

		const nameResult = v.safeParse(
			v.pipe(v.string(), v.trim(), v.minLength(1), v.maxLength(120)),
			bankName
		)
		if (!nameResult.success) return

		try {
			await mutation.mutateAsync({
				courseId,
				name: nameResult.output,
				questions,
			})
			onsuccess()
		} catch {
			// handled by parent
		}
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		{onclick}
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
