<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import {
		createForm,
		Field,
		Form,
		type SubmitEventHandler,
		reset,
	} from "@formisch/svelte"
	import { toast } from "svelte-sonner"
	import { fade, scale } from "svelte/transition"
	import { Plus, Trash2, CircleStop, Eye, AlertCircle, X } from "lucide-svelte"
	import { banksService } from "$lib/banks/banks.service"
	import { coursesService } from "$lib/courses/courses.service"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"
	import { createQuizSchema } from "$lib/quizzes/quizzes.schema"
	import type { CreateQuizInput } from "$lib/quizzes/types"
	import { getErrorMessage } from "$lib/shared/errors"
	import { quizKindLabel } from "$lib/shared/labels"

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

	const quizzesQuery = createQuery(() => ({
		queryKey: ["quizzes", "managed", data.courseId],
		queryFn: async () => {
			const all = await quizzesService.listManagedOrThrow()
			return all.filter(quiz => quiz.course.id === data.courseId)
		},
	}))

	const createQuizMutation = createMutation(() => ({
		mutationFn: (input: CreateQuizInput) => quizzesService.createOrThrow(input),
		onSuccess: async created => {
			toast.success(`Quiz ${created.title} creado.`)
			selectedBankIds = []
			reset(createQuizForm, {
				initialInput: {
					title: "",
					kind: "traditional",
					startsAt: initialStartsAt,
					attemptDurationMinutes: "30",
					questionCount: "10",
				},
			})
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "managed", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const deleteQuizMutation = createMutation(() => ({
		mutationFn: (quizId: string) => quizzesService.removeOrThrow(quizId),
		onSuccess: async () => {
			toast.success("Quiz eliminado correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "managed", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const closeAndPublishMutation = createMutation(() => ({
		mutationFn: (quizId: string) => quizzesService.closeAndPublishOrThrow(quizId),
		onSuccess: async () => {
			toast.success("Quiz finalizado y resultados publicados.")
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "managed", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

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

	const createQuizForm = createForm({
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

	const submitCreateQuiz: SubmitEventHandler<
		typeof createQuizSchema
	> = async output => {
		if (selectedBankIds.length === 0) {
			toast.error("Debes seleccionar al menos un banco.")
			return
		}

		const startsAtDate = new Date(output.startsAt)
		if (isNaN(startsAtDate.getTime())) {
			toast.error("Selecciona una fecha de inicio válida.")
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

		await createQuizMutation.mutateAsync({
			courseId: data.courseId,
			title: output.title,
			kind: output.kind,
			startsAt: startsAtDate.toISOString(),
			attemptDurationMinutes: Number(output.attemptDurationMinutes),
			questionCount: Number(output.questionCount),
			bankIds: selectedBankIds,
			certaintyConfig,
		})
		showCreateModal = false
	}

	const isActionPending = $derived(
		deleteQuizMutation.isPending || closeAndPublishMutation.isPending
	)

	let showCreateModal = $state(false)
	let copiedJoinCode = $state<string | null>(null)
	let quizToDelete = $state<{ id: string; title: string } | null>(null)

	const copyJoinCode = async (joinCode: string) => {
		await navigator.clipboard.writeText(joinCode)
		copiedJoinCode = joinCode
		setTimeout(() => {
			if (copiedJoinCode === joinCode) copiedJoinCode = null
		}, 1200)
		toast.success("Código copiado al portapapeles.")
	}

	const confirmDeleteQuiz = async () => {
		if (!quizToDelete) return
		await deleteQuizMutation.mutateAsync(quizToDelete.id)
		quizToDelete = null
	}
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="mt-2 mb-0 text-xl text-black">
					{courseQuery.data?.name ?? "Curso"} - Quizzes
				</h3>
				<p class="m-0 mt-2 text-zinc-700">
					Programa evaluaciones, monitorea su estado y publica resultados cuando
					corresponda.
				</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showCreateModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Nuevo quiz
			</button>
		</div>
	</header>

	<section class="panel-elevated p-4">
		{#if quizzesQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando quizzes...</p>
		{:else if quizzesQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(quizzesQuery.error)}</p>
		{:else if !quizzesQuery.data?.length}
			<p class="m-0 text-zinc-600">Aún no existen quizzes para este curso.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Título</th>
							<th class="px-3 py-2 text-left font-medium">Tipo</th>
							<th class="px-3 py-2 text-left font-medium">Estado</th>
							<th class="px-3 py-2 text-left font-medium">Código de acceso</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each quizzesQuery.data as quiz (quiz.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() =>
									goto(
										resolve(`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`)
									)}
							>
								<td class="px-3 py-2 text-zinc-900">{quiz.title}</td>
								<td class="px-3 py-2 text-zinc-700">{quizKindLabel(quiz.kind)}</td>
								<td class="px-3 py-2 text-zinc-700">
									{#if quiz.closedAt}
										<span
											class="inline-flex items-center gap-1 rounded-[4px] bg-zinc-900 px-2 py-1 text-xs text-white"
											><CircleStop size={12} aria-hidden="true" />Cerrado</span
										>
									{:else}
										<span class="pill-outline"
											><AlertCircle size={12} aria-hidden="true" />Abierto</span
										>
									{/if}
								</td>
								<td class="px-3 py-2 text-zinc-700">
									<button
										type="button"
										class="code-chip"
										title="Copiar código"
										onclick={e => {
											e.stopPropagation()
											void copyJoinCode(quiz.joinCode)
										}}
									>
										{copiedJoinCode === quiz.joinCode ? "Copiado" : quiz.joinCode}
									</button>
								</td>
								<td class="px-3 py-2">
									<div class="flex items-center gap-1">
										<a
											class="icon-btn"
											title="Ver intentos"
											onclick={e => e.stopPropagation()}
											href={resolve(
												`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`
											)}><Eye size={15} aria-hidden="true" /></a
										>
										{#if !quiz.resultsPublishedAt}
											<button
												class="icon-btn border-amber-300 text-amber-800 hover:bg-amber-50"
												title="Finalizar y publicar"
												type="button"
												onclick={e => {
													e.stopPropagation()
													closeAndPublishMutation.mutate(quiz.id)
												}}
												disabled={isActionPending || !!quiz.closedAt}
												><CircleStop size={15} aria-hidden="true" /></button
											>
										{/if}
										<button
											class="icon-btn icon-btn-danger"
											title="Eliminar"
											type="button"
											onclick={e => {
												e.stopPropagation()
												quizToDelete = { id: quiz.id, title: quiz.title }
											}}
											disabled={isActionPending}
											><Trash2 size={15} aria-hidden="true" /></button
										>
									</div>
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
				class="panel-elevated w-full max-w-3xl p-5"
				role="presentation"
				tabindex="-1"
				transition:scale={{ duration: 190, start: 0.98 }}
				onclick={e => e.stopPropagation()}
			>
				<div class="mb-3 flex items-center justify-between gap-2">
					<h4 class="m-0 text-base text-black">Crear quiz</h4>
					<button
						class="btn-tertiary p-1"
						type="button"
						onclick={() => (showCreateModal = false)}
						><X size={18} aria-hidden="true" /></button
					>
				</div>
				<Form of={createQuizForm} onsubmit={submitCreateQuiz} class="form-stack">
					<Field of={createQuizForm} path={["title"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<span class="text-sm text-zinc-800">Título</span>
								<input
									{...field.props}
									class="input-base"
									value={field.input ?? ""}
								/>
								{#if field.errors?.[0]}
									<span class="text-sm text-red-700">{field.errors[0]}</span>
								{/if}
							</label>
						{/snippet}
					</Field>

					<div class="form-grid-2 xl:grid-cols-3">
						<Field of={createQuizForm} path={["kind"]}>
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

						<Field of={createQuizForm} path={["attemptDurationMinutes"]}>
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

						<Field of={createQuizForm} path={["questionCount"]}>
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

					<Field of={createQuizForm} path={["startsAt"]}>
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
						{#if banksQuery.isLoading}
							<p class="m-0 text-sm text-zinc-600">Cargando bancos...</p>
						{:else if banksQuery.error}
							<p class="m-0 text-sm text-red-700">
								{getErrorMessage(banksQuery.error)}
							</p>
						{:else if !banksQuery.data?.length}
							<p class="m-0 text-sm text-zinc-600">No hay bancos disponibles.</p>
						{:else}
							<div class="grid gap-2 sm:grid-cols-2">
								{#each banksQuery.data as bank (bank.id)}
									<label
										class="flex items-center gap-2 rounded-[4px] border border-zinc-300 bg-white px-3 py-2"
									>
										<input
											type="checkbox"
											checked={selectedBankIds.includes(bank.id)}
											onchange={event =>
												toggleBank(
													bank.id,
													(event.currentTarget as HTMLInputElement).checked
												)}
										/>
										<span class="text-sm text-zinc-800">{bank.name}</span>
									</label>
								{/each}
							</div>
						{/if}
					</div>

					<div class="flex justify-end gap-2">
						<button
							class="btn-tertiary"
							type="button"
							onclick={() => (showCreateModal = false)}>Cancelar</button
						>
						<button class="btn-primary flex items-center gap-1.5" type="submit">
							<Plus size={16} aria-hidden="true" />
							{createQuizMutation.isPending ? "Creando..." : "Crear quiz"}
						</button>
					</div>
				</Form>
			</section>
		</div>
	{/if}

	<ConfirmActionModal
		open={!!quizToDelete}
		title="Eliminar quiz"
		message={quizToDelete ? `Se eliminara el quiz ${quizToDelete.title}.` : ""}
		confirmLabel="Eliminar"
		isPending={deleteQuizMutation.isPending}
		onCancel={() => (quizToDelete = null)}
		onConfirm={confirmDeleteQuiz}
	/>
</section>
