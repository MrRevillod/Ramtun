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
	import { Plus, RefreshCw, Users, Trash2, X } from "lucide-svelte"
	import {
		createCourseSchema,
		type CreateCourseFormValues,
	} from "$lib/courses/courses.schema"
	import { coursesService } from "$lib/courses/courses.service"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"
	import { getErrorMessage } from "$lib/shared/errors"

	const queryClient = useQueryClient()
	const coursesKey = ["courses"]

	const coursesQuery = createQuery(() => ({
		queryKey: coursesKey,
		queryFn: async () => {
			const courses = await coursesService.listOrThrow()
			const memberCounts = await Promise.all(
				courses.map(async course => {
					const members = await coursesService.listMembersOrThrow(course.id)
					return [course.id, members.length] as const
				})
			)
			const byId = new Map(memberCounts)
			return courses.map(course => ({
				...course,
				memberCount: byId.get(course.id) ?? 0,
			}))
		},
	}))

	const createCourseMutation = createMutation(() => ({
		mutationFn: (input: CreateCourseFormValues) =>
			coursesService.createOrThrow(input),
		onSuccess: async created => {
			toast.success(`Curso ${created.code} creado correctamente.`)
			reset(createCourseForm)
			await queryClient.invalidateQueries({ queryKey: coursesKey })
		},
		onError: error => {
			toast.error(getErrorMessage(error))
		},
	}))

	const deleteCourseMutation = createMutation(() => ({
		mutationFn: (courseId: string) => coursesService.removeOrThrow(courseId),
		onSuccess: async () => {
			toast.success("Curso eliminado correctamente.")
			courseToDelete = null
			await queryClient.invalidateQueries({ queryKey: coursesKey })
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const createCourseForm = createForm({
		schema: createCourseSchema,
		initialInput: {
			name: "",
			code: "",
			year: new Date().getFullYear(),
		},
	})

	const submitCourse: SubmitEventHandler<
		typeof createCourseSchema
	> = async output => {
		await createCourseMutation.mutateAsync(output)
		showCreateModal = false
	}

	let showCreateModal = $state(false)
	let courseToDelete = $state<{ id: string; name: string } | null>(null)

	const confirmDeleteCourse = async () => {
		if (!courseToDelete) return
		await deleteCourseMutation.mutateAsync(courseToDelete.id)
	}
</script>

<section class="grid gap-5">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h2 class="mt-2 mb-0 text-2xl text-black">Gestionar Mis Cursos</h2>
				<p class="mt-2 max-w-3xl text-zinc-700">
					Crea cursos, organiza participantes y prepara evaluaciones para cada
					sección.
				</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showCreateModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Nuevo curso
			</button>
		</div>
	</header>

	<section class="panel-elevated p-4 sm:p-5">

		{#if coursesQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando cursos...</p>
		{:else if coursesQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(coursesQuery.error)}</p>
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
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each coursesQuery.data as course (course.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() => goto(resolve(`/courses/${course.id}/quizzes`))}
							>
								<td class="px-3 py-2 text-zinc-800">{course.name}</td>
								<td class="px-3 py-2 font-medium text-zinc-900">{course.code}</td>
								<td class="px-3 py-2 text-zinc-700">{course.year}</td>
								<td class="px-3 py-2 text-zinc-700">
									<span class="inline-flex items-center gap-1">
										<Users size={14} aria-hidden="true" />
										{course.memberCount}
									</span>
								</td>
								<td class="px-3 py-2">
									<div class="flex items-center gap-1">
										<button
											class="icon-btn icon-btn-danger"
											title="Eliminar curso"
											type="button"
											onclick={e => {
												e.stopPropagation()
												courseToDelete = { id: course.id, name: course.name }
											}}
										>
											<Trash2 size={15} aria-hidden="true" />
										</button>
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
				class="panel-elevated w-full max-w-xl p-5 sm:p-6"
				role="presentation"
				tabindex="-1"
				transition:scale={{ duration: 190, start: 0.98 }}
				onclick={e => e.stopPropagation()}
			>
				<div class="mb-3 flex items-center justify-between gap-2">
					<h3 class="m-0 text-lg text-black">Crear curso</h3>
					<button
						class="btn-tertiary p-1"
						type="button"
						onclick={() => (showCreateModal = false)}
						><X size={18} aria-hidden="true" /></button
					>
				</div>
				<Form of={createCourseForm} onsubmit={submitCourse} class="form-stack">
					<Field of={createCourseForm} path={["name"]}
						>{#snippet children(field)}<label class="grid gap-1.5"
								><span class="text-sm text-zinc-800">Nombre</span><input
									{...field.props}
									class="input-base"
									value={field.input ?? ""}
									placeholder="Ej: Matematica Discreta"
								/>{#if field.errors?.[0]}<span class="text-sm text-red-700"
										>{field.errors[0]}</span
									>{/if}</label
							>{/snippet}</Field
					>
					<div class="form-grid-2">
						<Field of={createCourseForm} path={["code"]}
							>{#snippet children(field)}<label class="grid gap-1.5"
									><span class="text-sm text-zinc-800">Código</span><input
										{...field.props}
										class="input-base"
										value={field.input ?? ""}
										placeholder="Ej: MAT123"
									/>{#if field.errors?.[0]}<span class="text-sm text-red-700"
											>{field.errors[0]}</span
										>{/if}</label
								>{/snippet}</Field
						>
						<Field of={createCourseForm} path={["year"]}
							>{#snippet children(field)}<label class="grid gap-1.5"
									><span class="text-sm text-zinc-800">Año</span><input
										{...field.props}
										class="input-base"
										type="number"
										value={field.input ?? ""}
									/>{#if field.errors?.[0]}<span class="text-sm text-red-700"
											>{field.errors[0]}</span
										>{/if}</label
								>{/snippet}</Field
						>
					</div>
					<div class="flex justify-end gap-2">
						<button
							class="btn-tertiary"
							type="button"
							onclick={() => (showCreateModal = false)}>Cancelar</button
						><button class="btn-primary flex items-center gap-1.5" type="submit"
							>{#if createCourseMutation.isPending}<span class="animate-spin"
									><RefreshCw size={16} aria-hidden="true" /></span
								>{:else}<Plus
									size={16}
									aria-hidden="true"
								/>{/if}{createCourseMutation.isPending
								? "Creando..."
								: "Crear curso"}</button
						>
					</div>
				</Form>
			</section>
		</div>
	{/if}

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
