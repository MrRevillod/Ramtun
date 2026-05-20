<script lang="ts">
	import { fade, scale } from "svelte/transition"
	import {
		createForm,
		Field,
		Form,
		type SubmitEventHandler,
		reset,
	} from "@formisch/svelte"
	import { Plus, X } from "lucide-svelte"
	import type { CreateQuizInput } from "$lib/quizzes/quizzes.dtos"
	import { createQuizSchema } from "$lib/quizzes/quizzes.dtos"
	import BankSelector from "$lib/quizzes/components/BankSelector.svelte"
	import { toast } from "svelte-sonner"
	import { getErrorMessage } from "$lib/shared/errors"

	interface CreateQuizModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: (created: { title: string }) => void
		mutation: {
			isPending: boolean
			mutateAsync: (input: CreateQuizInput) => Promise<{ title: string }>
		}
	}

	let { open, courseId, onclose, onsuccess, mutation }: CreateQuizModalProps =
		$props()

	const toLocalDatetimeInput = (date: Date): string => {
		const y = date.getFullYear()
		const m = String(date.getMonth() + 1).padStart(2, "0")
		const d = String(date.getDate()).padStart(2, "0")
		const h = String(date.getHours()).padStart(2, "0")
		const min = String(date.getMinutes()).padStart(2, "0")
		return `${y}-${m}-${d}T${h}:${min}`
	}

	const now = new Date()
	const initialStartsAt = toLocalDatetimeInput(
		new Date(now.getTime() + 5 * 60 * 1000)
	)

	const form = createForm({
		schema: createQuizSchema,
		initialInput: {
			title: "",
			kind: "traditional",
			startsAt: initialStartsAt,
			attemptDurationMinutes: "30",
			questionCount: "10",
		},
	})

	let selectedBankIds = $state<string[]>([])

	const toggleBank = (bankId: string, selected: boolean) => {
		selectedBankIds = selected
			? [...selectedBankIds, bankId]
			: selectedBankIds.filter(id => id !== bankId)
	}

	const handleSubmit: SubmitEventHandler<typeof createQuizSchema> = async output => {
		try {
			if (selectedBankIds.length === 0) {
				toast.error("Selecciona al menos un banco.")
				return
			}

			const isCertainty = output.kind === "certainty"

			const certaintyConfig = isCertainty
				? {
						low: { correct: 1, incorrect: 0 },
						medium: { correct: 2, incorrect: -1 },
						high: { correct: 3, incorrect: -2 },
					}
				: null

			const created = await mutation.mutateAsync({
				courseId,
				title: output.title,
				kind: output.kind,
				startsAt: output.startsAt,
				attemptDurationMinutes: output.attemptDurationMinutes,
				questionCount: output.questionCount,
				bankIds: selectedBankIds,
				certaintyConfig,
			})

			selectedBankIds = []
			reset(form, {
				initialInput: {
					title: "",
					kind: "traditional",
					startsAt: initialStartsAt,
					attemptDurationMinutes: "30",
					questionCount: "10",
				},
			})
			onsuccess(created)
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
		transition:fade={{ duration: 180 }}
		onclick={() => onclose()}
		onkeydown={e => {
			if (e.key === "Escape") onclose()
		}}
	>
		<section
			class="panel-elevated w-full max-w-3xl p-5"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={e => e.stopPropagation()}
		>
			<div class="mb-3 flex items-center justify-between gap-2">
				<h4 class="m-0 text-base text-black">Crear quiz</h4>
				<button class="btn-tertiary p-1" type="button" onclick={onclose}>
					<X size={18} aria-hidden="true" />
				</button>
			</div>

			<Form of={form} onsubmit={handleSubmit} class="form-stack">
				<Field of={form} path={["title"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Título</span>
							<input {...field.props} class="input-base" value={field.input ?? ""} />
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>

				<div class="form-grid-2 xl:grid-cols-3">
					<Field of={form} path={["kind"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Tipo</span>
								<select
									{...field.props}
									class="input-base"
									value={field.input ?? "traditional"}
								>
									<option value="traditional">Tradicional</option>
									<option value="certainty">Certeza</option>
								</select>
							</label>
						{/snippet}
					</Field>

					<Field of={form} path={["attemptDurationMinutes"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Duracion (min)</span>
								<input
									{...field.props}
									type="number"
									class="input-base"
									value={field.input ?? 30}
								/>
							</label>
						{/snippet}
					</Field>

					<Field of={form} path={["questionCount"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Cantidad preguntas</span>
								<input
									{...field.props}
									type="number"
									class="input-base"
									value={field.input ?? 10}
								/>
							</label>
						{/snippet}
					</Field>
				</div>

				<Field of={form} path={["startsAt"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5 sm:max-w-sm">
							<span class="text-sm text-zinc-800">Inicio</span>
							<input
								{...field.props}
								type="datetime-local"
								class="input-base"
								value={field.input ?? ""}
							/>
						</label>
					{/snippet}
				</Field>

				<div class="grid gap-2">
					<span class="text-sm text-zinc-800">Bancos fuente</span>
					<BankSelector {courseId} {selectedBankIds} onchange={toggleBank} />
				</div>

				<div class="flex justify-end gap-2">
					<button class="btn-tertiary" type="button" onclick={onclose}
						>Cancelar</button
					>
					<button
						class="btn-primary flex items-center gap-1.5"
						type="submit"
						disabled={mutation.isPending}
					>
						<Plus size={16} aria-hidden="true" />
						{mutation.isPending ? "Creando..." : "Crear quiz"}
					</button>
				</div>
			</Form>
		</section>
	</div>
{/if}
