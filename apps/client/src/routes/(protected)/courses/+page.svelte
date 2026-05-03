<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { resolve } from "$app/paths"
	import { createForm, Field, Form, type SubmitEventHandler, reset } from "@formisch/svelte"
	import { toast } from "svelte-sonner"
	import { Plus, RefreshCw, FolderOpen } from "lucide-svelte"
	import {
		createCourseSchema,
		type CreateCourseFormValues,
	} from "$lib/courses/courses.schema"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"

	const queryClient = useQueryClient()
	const coursesKey = ["courses"]

	const coursesQuery = createQuery(() => ({
		queryKey: coursesKey,
		queryFn: () => coursesService.listOrThrow(),
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
	}
</script>

<section class="grid gap-5">
	<header>
		<p class="section-kicker m-0">Cursos</p>
		<h2 class="mt-2 mb-0 text-2xl text-black">Cursos</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Crea cursos y administra sus miembros, bancos y quizzes.
		</p>
	</header>

	<div class="grid gap-4 xl:grid-cols-[minmax(22rem,28rem)_minmax(0,1fr)] xl:items-start">
	<section class="panel-muted p-4 sm:p-5">
		<h3 class="m-0 text-lg text-black">Crear curso</h3>
		<Form of={createCourseForm} onsubmit={submitCourse} class="mt-3 grid gap-3">
			<Field of={createCourseForm} path={["name"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-sm text-zinc-800">Nombre</span>
						<input
							{...field.props}
							class="input-base"
							value={field.input ?? ""}
							placeholder="Ej: Matematica Discreta"
						/>
						{#if field.errors?.[0]}
							<span class="text-sm text-red-700">{field.errors[0]}</span>
						{/if}
					</label>
				{/snippet}
			</Field>

			<div class="grid gap-3 sm:grid-cols-2">
				<Field of={createCourseForm} path={["code"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Codigo</span>
							<input
								{...field.props}
								class="input-base"
								value={field.input ?? ""}
								placeholder="Ej: MAT123"
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>

				<Field of={createCourseForm} path={["year"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Año</span>
							<input
								{...field.props}
								class="input-base"
								type="number"
								value={field.input ?? ""}
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>
			</div>

			<button
				class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
				type="submit"
			>
				{#if createCourseMutation.isPending}
					<span class="animate-spin">
						<RefreshCw size={16} aria-hidden="true" />
					</span>
				{:else}
					<Plus size={16} aria-hidden="true" />
				{/if}
				{createCourseMutation.isPending ? "Creando..." : "Crear curso"}
			</button>
		</Form>
	</section>

	<section class="panel-surface p-4 sm:p-5">
		<div class="mb-3 flex items-center justify-between gap-3">
			<h3 class="m-0 text-lg text-black">Mis cursos</h3>
			<button
				class="btn-secondary flex items-center gap-1.5"
				type="button"
				onclick={() => coursesQuery.refetch()}
				disabled={coursesQuery.isFetching}
			>
				<RefreshCw size={16} aria-hidden="true" />
				Actualizar
			</button>
		</div>

		{#if coursesQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando cursos...</p>
		{:else if coursesQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(coursesQuery.error)}</p>
		{:else if !coursesQuery.data?.length}
			<p class="m-0 text-zinc-600">No tienes cursos creados.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="bg-zinc-100/90 text-zinc-700">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Codigo</th>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Año</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each coursesQuery.data as course (course.id)}
							<tr class="border-t border-zinc-200 bg-white/80">
								<td class="px-3 py-2 font-medium text-zinc-900">{course.code}</td>
								<td class="px-3 py-2 text-zinc-800">{course.name}</td>
								<td class="px-3 py-2 text-zinc-700">{course.year}</td>
								<td class="px-3 py-2">
									<a
										class="btn-tertiary flex items-center gap-1.5"
										href={resolve(`/courses/${course.id}/members`)}
									>
										<FolderOpen size={16} aria-hidden="true" />
										Abrir
									</a>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
	</div>
</section>
