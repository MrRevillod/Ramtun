<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { createForm, Field, Form, type SubmitEventHandler, reset } from "@formisch/svelte"
	import { toast } from "svelte-sonner"
	import { Plus, RefreshCw, UserMinus } from "lucide-svelte"
	import {
		addCourseMemberSchema,
		type AddCourseMemberFormValues,
	} from "$lib/courses/courses.schema"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { usersService } from "$lib/users/users.service"
	import { roleLabel } from "$lib/shared/labels"

	let { data } = $props()

	const queryClient = useQueryClient()

	const courseQuery = createQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.getOrThrow(data.courseId),
	}))

	const membersQuery = createQuery(() => ({
		queryKey: ["course-members", data.courseId],
		queryFn: () => coursesService.listMembersOrThrow(data.courseId),
	}))

	const candidatesQuery = createQuery(() => ({
		queryKey: ["collaborator-candidates", data.courseId],
		queryFn: () => usersService.listCollaboratorCandidatesOrThrow(),
	}))

	const addMemberMutation = createMutation(() => ({
		mutationFn: (input: AddCourseMemberFormValues) =>
			coursesService.addMemberOrThrow(data.courseId, input),
		onSuccess: async () => {
			toast.success("Miembro agregado correctamente.")
			reset(addMemberForm)
			await queryClient.invalidateQueries({
				queryKey: ["course-members", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const removeMemberMutation = createMutation(() => ({
		mutationFn: (userId: string) =>
			coursesService.removeMemberOrThrow(data.courseId, userId),
		onSuccess: async () => {
			toast.success("Miembro removido correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["course-members", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const addMemberForm = createForm({
		schema: addCourseMemberSchema,
		initialInput: { userId: "" },
	})

	const submitAddMember: SubmitEventHandler<
		typeof addCourseMemberSchema
	> = async output => {
		await addMemberMutation.mutateAsync(output)
	}
</script>

<section class="grid gap-4">
	<header>
		<h3 class="mt-2 mb-0 text-xl text-black">
			Miembros {#if courseQuery.data}<span class="text-zinc-500"
					>· {courseQuery.data.code}</span
				>{/if}
		</h3>
		<p class="mt-2 mb-0 text-zinc-700">Administra los integrantes del curso.</p>
	</header>

	<section class="panel-muted p-4">
		<h4 class="m-0 text-base text-black">Agregar miembro</h4>
		<Form of={addMemberForm} onsubmit={submitAddMember} class="mt-3 grid gap-3">
			<Field of={addMemberForm} path={["userId"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-sm text-zinc-800">Usuario</span>
						<select {...field.props} class="input-base" value={field.input ?? ""}>
							<option value="">Selecciona un usuario</option>
							{#each candidatesQuery.data ?? [] as candidate (candidate.id)}
								<option value={candidate.id}>
									{candidate.username} · {candidate.name}
								</option>
							{/each}
						</select>
						{#if field.errors?.[0]}
							<span class="text-sm text-red-700">{field.errors[0]}</span>
						{/if}
					</label>
				{/snippet}
			</Field>

			<button
				class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
				type="submit"
			>
				<Plus size={16} aria-hidden="true" />
				{addMemberMutation.isPending ? "Agregando..." : "Agregar miembro"}
			</button>
		</Form>
	</section>

	<section class="panel-surface p-4">
		<div class="mb-3 flex items-center justify-between gap-3">
			<h4 class="m-0 text-base text-black">Listado de miembros</h4>
			<button
				class="btn-secondary flex items-center gap-1.5"
				type="button"
				onclick={() => membersQuery.refetch()}
				disabled={membersQuery.isFetching}
			>
				<RefreshCw size={16} aria-hidden="true" />
				Actualizar
			</button>
		</div>

		{#if membersQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando miembros...</p>
		{:else if membersQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(membersQuery.error)}</p>
		{:else if !membersQuery.data?.length}
			<p class="m-0 text-zinc-600">El curso aun no tiene miembros.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="bg-zinc-100/90 text-zinc-700">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Username</th>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Rol</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each membersQuery.data as member (member.userId)}
							<tr class="border-t border-zinc-200 bg-white/80">
								<td class="px-3 py-2 font-medium text-zinc-900">{member.username}</td
								>
								<td class="px-3 py-2 text-zinc-800">{member.name}</td>
								<td class="px-3 py-2 text-zinc-700">{roleLabel(member.role)}</td>
								<td class="px-3 py-2">
									<button
										class="btn-tertiary flex items-center gap-1.5"
										type="button"
										onclick={() => removeMemberMutation.mutate(member.userId)}
										disabled={removeMemberMutation.isPending}
									>
										<UserMinus size={16} aria-hidden="true" />
										Quitar
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>
