<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { Plus, Trash2, Eye } from "lucide-svelte"
	import { CircleStop } from "lucide-svelte"
	import { coursesService } from "$lib/courses/courses.service"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import type { CreateQuizInput } from "$lib/quizzes/quizzes.dtos"
	import { getErrorMessage } from "$lib/shared/errors"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"
	import CreateQuizModal from "$lib/quizzes/components/CreateQuizModal.svelte"
	import QuizStatusBadge from "$lib/quizzes/components/QuizStatusBadge.svelte"
	import { QuizKindValue } from "$lib/shared/value-objects/quiz-kind.values.js"

	let { data } = $props()

	const queryClient = useQueryClient()

	const courseQuery = createQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.get(data.courseId),
	}))

	const quizzesQuery = createQuery(() => ({
		queryKey: ["quizzes", "managed", data.courseId],
		queryFn: async () => {
			const all = await quizzesService.listManaged()
			return all.filter(quiz => quiz.course.id === data.courseId)
		},
	}))

	const createQuizMutation = createMutation(() => ({
		mutationFn: (input: CreateQuizInput) => quizzesService.create(input),
		onSuccess: async created => {
			toast.success(`Quiz ${created.title} creado.`)
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "managed", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const deleteQuizMutation = createMutation(() => ({
		mutationFn: (quizId: string) => quizzesService.remove(quizId),
		onSuccess: async () => {
			toast.success("Quiz eliminado correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "managed", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const closeAndPublishMutation = createMutation(() => ({
		mutationFn: (quizId: string) => quizzesService.closeAndPublish(quizId),
		onSuccess: async () => {
			toast.success("Quiz finalizado y resultados publicados.")
			await queryClient.invalidateQueries({
				queryKey: ["quizzes", "managed", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

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

	const isActionPending = $derived(
		deleteQuizMutation.isPending || closeAndPublishMutation.isPending
	)
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
				class="btn-primary flex cursor-pointer items-center gap-1.5"
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
								<td class="px-3 py-2 text-zinc-700"
									>{QuizKindValue.format(quiz.kind)}</td
								>
								<td class="px-3 py-2 text-zinc-700">
									<QuizStatusBadge closedAt={quiz.closedAt} />
								</td>
								<td class="px-3 py-2 text-zinc-700">
									<button
										type="button"
										class="code-chip cursor-pointer"
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
											class="icon-btn cursor-pointer"
											title="Ver intentos"
											onclick={e => e.stopPropagation()}
											href={resolve(
												`/courses/${data.courseId}/quizzes/${quiz.id}/attempts`
											)}
										>
											<Eye size={15} aria-hidden="true" /></a
										>
										{#if !quiz.resultsPublishedAt}
											<button
												class="icon-btn cursor-pointer border-amber-300 text-amber-800 hover:bg-amber-50"
												title="Finalizar y publicar"
												type="button"
												onclick={e => {
													e.stopPropagation()
													closeAndPublishMutation.mutate(quiz.id)
												}}
												disabled={isActionPending || !!quiz.closedAt}
											>
												<CircleStop size={15} aria-hidden="true" /></button
											>
										{/if}
										<button
											class="icon-btn icon-btn-danger cursor-pointer"
											title="Eliminar"
											type="button"
											onclick={e => {
												e.stopPropagation()
												quizToDelete = { id: quiz.id, title: quiz.title }
											}}
											disabled={isActionPending}
										>
											<Trash2 size={15} aria-hidden="true" /></button
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

	<CreateQuizModal
		open={showCreateModal}
		courseId={data.courseId}
		onclose={() => (showCreateModal = false)}
		onsuccess={() => (showCreateModal = false)}
		mutation={createQuizMutation}
	/>

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
