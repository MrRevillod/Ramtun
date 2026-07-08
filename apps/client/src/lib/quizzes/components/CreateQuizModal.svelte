<script lang="ts">
	import { getInput, setInput, type SubmitEventHandler } from "@formisch/svelte"
	import { toast } from "svelte-sonner"
	import { Plus, X } from "lucide-svelte"
	import { fade, scale } from "svelte/transition"
	import { untrack } from "svelte"
	import { queryClient, useMutation } from "$lib/shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { CertaintyValue } from "$lib/shared/value-objects/certainty.value"
	import type * as v from "valibot"

	import DateInput from "$lib/shared/components/DateInput.svelte"
	import BankSelector from "$lib/quizzes/components/BankSelector.svelte"
	import { quizzesService } from "../service"
	import {
		createQuizDTOSchema,
		type CertaintyTableDTO,
		type CreateQuizDTO,
		type CreateQuizDTOSchema,
	} from "$lib/quizzes/dtos"

	interface CreateQuizModalProps {
		open: boolean
		courseId: string
		onclose: () => void
		onsuccess: () => void
	}

	let { open, courseId, onclose, onsuccess }: CreateQuizModalProps = $props()

	const mutation = useMutation(() => ({
		mutationFn: (data: CreateQuizDTO) => quizzesService.create(data),
		onSuccess: async (created) => {
			toast.success(`Cuestionario ${created.title} creado.`)
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "course", courseId],
			})
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
	}))

	const defaultCertainty: CertaintyTableDTO = {
		low: { correct: 1, incorrect: 0 },
		medium: { correct: 2, incorrect: -2 },
		high: { correct: 3, incorrect: -4 },
	}

	const initialInput = untrack(
		() =>
			({
				courseId,
				title: "",
				kind: "traditional" as const,
				startsAt: new Date(new Date().getFullYear() + 1, 0, 1).toISOString(),
				attemptDurationMinutes: "30",
				questionCount: "10",
				bankIds: [] as string[],
				certaintyConfig: null,
			}) satisfies v.InferInput<typeof createQuizDTOSchema>
	)

	const form = createForm({ schema: createQuizDTOSchema, initialInput })

	const kind = $derived(
		(getInput(form, { path: ["kind"] as any }) ?? "traditional") as "traditional" | "certainty"
	)
	const certaintyConfigValue = $derived(
		(getInput(form, { path: ["certaintyConfig"] as any }) ?? null) as CertaintyTableDTO | null
	)

	$effect(() => {
		if (kind === "traditional" && certaintyConfigValue !== null) {
			setInput(form, { path: ["certaintyConfig"] as any, input: null } as never)
		} else if (kind === "certainty" && !certaintyConfigValue) {
			setInput(
				form,
				{ path: ["certaintyConfig"] as any, input: defaultCertainty } as never
			)
		}
	})

	const toggleBank = (bankId: string, selected: boolean) => {
		const current = (getInput(form, { path: ["bankIds"] as any }) ?? []) as string[]
		const next = selected
			? [...current, bankId]
			: current.filter((id) => id !== bankId)
		setInput(form, { path: ["bankIds"] as any, input: next } as never)
	}

	const updateCertainty = (
		level: keyof CertaintyTableDTO,
		field: "correct" | "incorrect",
		value: number
	) => {
		const current = certaintyConfigValue ?? defaultCertainty
		setInput(form, {
			path: ["certaintyConfig"] as any,
			input: {
				...current,
				[level]: { ...current[level], [field]: value },
			},
		} as never)
	}

	const handleSubmit: SubmitEventHandler<CreateQuizDTOSchema> = async (output) => {
		await mutation.mutateAsync(output)
		reset(form, { initialInput })
		onsuccess()
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
		onkeydown={(e) => {
			if (e.key === "Escape") onclose()
		}}
	>
		<section
			class="panel-elevated w-full max-w-3xl p-5"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={(e) => e.stopPropagation()}
		>
			<div class="mb-3 flex items-center justify-between gap-2">
				<h4 class="m-0 text-base text-black">Crear cuestionario</h4>
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

				<div class="form-grid-2 grid-cols-2">
					<Field of={form} path={["kind"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Tipo</span>
								<select {...field.props} class="input-base" value={field.input ?? "traditional"}>
									<option value="traditional">Tradicional</option>
									<option value="certainty">Certeza</option>
								</select>
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

				<div class="form-grid-2 grid-cols-2">
					<label class="grid gap-1.5">
						<span class="text-sm text-zinc-800">Inicio</span>
						<DateInput {form} path={["startsAt"]} />
					</label>

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
				</div>

				<Field of={form} path={["bankIds"]}>
					{#snippet children(field)}
						<div class="grid gap-2">
							<span class="text-sm text-zinc-800">Bancos fuente</span>
							<BankSelector
								{courseId}
								selectedBankIds={field.input ?? []}
								onchange={toggleBank}
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</div>
					{/snippet}
				</Field>

				{#if kind === "certainty"}
					<div class="grid gap-3">
						<div class="flex items-center justify-between">
							<span class="text-sm text-zinc-800">Tabla de certeza</span>
							<span class="text-xs text-zinc-500">Puntaje por respuesta</span>
						</div>
						<div class="overflow-x-auto">
							<table class="w-full border-collapse text-xs">
								<thead class="bg-zinc-100/90 text-zinc-700">
									<tr>
										<th class="border border-zinc-300 px-2 py-1.5 text-left">Nivel</th>
										<th class="border border-zinc-300 px-2 py-1.5 text-left">Correcta</th>
										<th class="border border-zinc-300 px-2 py-1.5 text-left">Incorrecta</th>
									</tr>
								</thead>
								<tbody>
									{#each (["low", "medium", "high"] as const) as level (level)}
										<tr>
											<td class="border border-zinc-300 bg-white px-2 py-1.5">
												{CertaintyValue.format(level)}
											</td>
											<td class="border border-zinc-300 bg-white px-2 py-1.5">
												<input
													type="number"
													class="input-base h-8"
													value={certaintyConfigValue?.[level].correct ?? 0}
													oninput={(e) =>
														updateCertainty(level, "correct", Number(e.currentTarget.value))}
												/>
											</td>
											<td class="border border-zinc-300 bg-white px-2 py-1.5">
												<input
													type="number"
													class="input-base h-8"
													value={certaintyConfigValue?.[level].incorrect ?? 0}
													oninput={(e) =>
														updateCertainty(level, "incorrect", Number(e.currentTarget.value))}
												/>
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				{/if}

				<div class="flex justify-end gap-2">
					<button class="btn-tertiary" type="button" onclick={onclose}>Cancelar</button>
					<button
						class="btn-primary flex items-center gap-1.5"
						type="submit"
						disabled={mutation.isPending}
					>
						<Plus size={16} aria-hidden="true" />
						{mutation.isPending ? "Creando..." : "Crear cuestionario"}
					</button>
				</div>
			</Form>
		</section>
	</div>
{/if}
