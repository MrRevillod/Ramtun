<script lang="ts">
	import type { Course } from "$lib/courses/entity"

	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { authStore } from "$lib/auth/store.svelte"
	import { Plus, Users, Trash2 } from "lucide-svelte"
	import { coursesService } from "$lib/courses/service"
	import { useQuery, useMutation, useQueryClient } from "$lib/shared/http/tanstack"

	import CreateCourseModal from "$lib/courses/components/CreateCourseModal.svelte"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"

	let showCreateModal = $state(false)
	let courseToDelete = $state<Course | null>(null)
	const canCreateCourse = $derived(authStore.user?.isAdmin() || authStore.user?.isFunc())

	const queryClient = useQueryClient()

	const coursesQuery = useQuery(() => ({
		queryKey: ["courses"],
		queryFn: async () => await coursesService.list(),
	}))

	const deleteCourseMutation = useMutation(() => ({
		mutationFn: (courseId: string) => coursesService.remove(courseId),
		onSuccess: async () => {
			toast.success("Curso eliminado correctamente.")
			await queryClient.invalidateQueries({ queryKey: ["courses"] })
		},
		onError: (error) => {
			toast.error(error.messageOrDefault)
		},
		onSettled: () => {
			courseToDelete = null
		},
	}))

	const confirmDeleteCourse = async () => {
		await deleteCourseMutation.mutateAsync(courseToDelete?.id ?? "")
	}
</script>

<section class="grid gap-5">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h2 class="mt-2 mb-0 text-2xl text-black">
					{authStore.user?.isAdmin() ? "Cursos del Sistema" : "Gestionar Mis Cursos"}
				</h2>
				<p class="mt-2 max-w-3xl text-zinc-700">
					{authStore.user?.isAdmin()
						? "Vista general de todos los cursos registrados en la plataforma."
						: "Crea cursos, organiza participantes y prepara evaluaciones para cada sección."}
				</p>
			</div>
			{#if canCreateCourse}
				<button
					class="btn-primary flex cursor-pointer items-center gap-1.5"
					type="button"
					onclick={() => (showCreateModal = true)}
				>
					<Plus size={16} aria-hidden="true" />
					Nuevo curso
				</button>
			{/if}
		</div>
	</header>

	<section>
		{#if coursesQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando cursos...</p>
		{:else if coursesQuery.error}
			<p class="m-0 text-red-700">
				{coursesQuery.error?.messageOrDefault ?? ""}
			</p>
		{:else if !coursesQuery.data?.length}
			<p class="notice notice-warn m-0">Aún no tienes cursos creados.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Código</th>
							<th class="px-3 py-2 text-left font-medium">Año</th>
							<th class="px-3 py-2 text-left font-medium">Miembros</th>
							{#if canCreateCourse}
								<th class="px-3 py-2 text-left font-medium">Acciones</th>
							{/if}
						</tr>
					</thead>
					<tbody>
						{#each coursesQuery.data as course (course.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() => goto(`/courses/${course.id}/quizzes`)}
							>
								<td class="px-3 py-2 text-zinc-800">{course.name}</td>
								<td class="px-3 py-2 font-medium text-zinc-900">{course.code}</td>
								<td class="px-3 py-2 text-zinc-700">{course.year}</td>
								<td class="px-3 py-2 text-zinc-700">
									<span class="inline-flex items-center gap-1">
										<Users size={14} aria-hidden="true" />
										{course.members.length}
									</span>
								</td>
								{#if canCreateCourse}
									<td class="px-3 py-2">
										<div class="flex items-center gap-1">
											<button
												class="icon-btn icon-btn-danger cursor-pointer"
												title="Eliminar curso"
												type="button"
												onclick={(e) => {
													e.stopPropagation()
													courseToDelete = course
												}}
											>
												<Trash2 size={15} aria-hidden="true" />
											</button>
										</div>
									</td>
								{/if}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>

	<CreateCourseModal
		open={showCreateModal}
		onclose={() => (showCreateModal = false)}
		onsuccess={() => (showCreateModal = false)}
	/>

	<ConfirmActionModal
		open={!!courseToDelete}
		title="Eliminar curso"
		message={courseToDelete
			? `Se eliminara el curso ${courseToDelete.name}. Esta accion no se puede deshacer.`
			: ""}
		confirmLabel="Eliminar"
		isPending={deleteCourseMutation.isPending}
		onCancel={() => (courseToDelete = null)}
		onConfirm={confirmDeleteCourse}
	/>
</section>
