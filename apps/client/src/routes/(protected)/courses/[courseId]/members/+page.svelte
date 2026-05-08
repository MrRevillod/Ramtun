<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import {
		createForm,
		Field,
		Form,
		type SubmitEventHandler,
		reset,
	} from "@formisch/svelte"
	import { toast } from "svelte-sonner"
	import { fade, scale } from "svelte/transition"
	import { Plus, Trash2, X } from "lucide-svelte"
	import {
		addCourseMemberSchema,
		type AddCourseMemberFormValues,
	} from "$lib/courses/courses.schema"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { usersService } from "$lib/users/users.service"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"
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
		showAddModal = false
	}

	let showAddModal = $state(false)
	let memberToRemove = $state<{ userId: string; username: string } | null>(null)

	const confirmRemoveMember = async () => {
		if (!memberToRemove) return
		await removeMemberMutation.mutateAsync(memberToRemove.userId)
		memberToRemove = null
	}
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="mt-2 mb-0 text-xl text-black">
					{courseQuery.data?.name ?? "Curso"} - Miembros
				</h3>
				<p class="mt-2 mb-0 text-zinc-700">
					Agrega o retira participantes.
				</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showAddModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Agregar miembro
			</button>
		</div>
	</header>

	<section class="panel-elevated p-4">
		{#if membersQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando miembros...</p>
		{:else if membersQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(membersQuery.error)}</p>
		{:else if !membersQuery.data?.length}
			<p class="notice notice-warn m-0">El curso aún no tiene miembros.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Usuario</th>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Rol</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each membersQuery.data as member (member.userId)}
							<tr class="table-row">
								<td class="px-3 py-2 font-medium text-zinc-900">{member.username}</td
								>
								<td class="px-3 py-2 text-zinc-800">{member.name}</td>
								<td class="px-3 py-2 text-zinc-700">{roleLabel(member.role)}</td>
								<td class="px-3 py-2">
									<button
									class="icon-btn icon-btn-danger"
									title="Quitar"
									type="button"
									onclick={() =>
										(memberToRemove = {
											userId: member.userId,
											username: member.username,
										})}
									disabled={removeMemberMutation.isPending}
								>
										<Trash2 size={15} aria-hidden="true" />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>

	{#if showAddModal}
		<div
			class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
			transition:fade={{ duration: 180 }}
			onclick={() => (showAddModal = false)}
			onkeydown={e => {
				if (e.key === "Escape") showAddModal = false
			}}
		>
			<section
				class="panel-elevated w-full max-w-xl p-5"
				role="presentation"
				tabindex="-1"
				transition:scale={{ duration: 190, start: 0.98 }}
				onclick={e => e.stopPropagation()}
			>
				<div class="mb-3 flex items-center justify-between gap-2">
					<h4 class="m-0 text-base text-black">Agregar miembro</h4>
					<button
						class="btn-tertiary p-1"
						type="button"
						onclick={() => (showAddModal = false)}
						><X size={18} aria-hidden="true" /></button
					>
				</div>
				<Form of={addMemberForm} onsubmit={submitAddMember} class="form-stack">
					<Field of={addMemberForm} path={["userId"]}
						>{#snippet children(field)}<label class="grid gap-1.5"
								><span class="text-sm text-zinc-800">Usuario</span><select
									{...field.props}
									class="input-base"
									value={field.input ?? ""}
									><option value="">Selecciona un usuario</option
									>{#each candidatesQuery.data ?? [] as candidate (candidate.id)}<option
											value={candidate.id}
											>{candidate.username} · {candidate.name}</option
										>{/each}</select
								>{#if field.errors?.[0]}<span class="text-sm text-red-700"
										>{field.errors[0]}</span
									>{/if}</label
							>{/snippet}</Field
					>
					<div class="flex justify-end gap-2">
						<button
							class="btn-tertiary"
							type="button"
							onclick={() => (showAddModal = false)}>Cancelar</button
						><button class="btn-primary flex items-center gap-1.5" type="submit"
							><Plus size={16} aria-hidden="true" />{addMemberMutation.isPending
								? "Agregando..."
								: "Agregar miembro"}</button
						>
					</div>
				</Form>
			</section>
		</div>
	{/if}

	<ConfirmActionModal
		open={!!memberToRemove}
		title="Quitar miembro"
		message={memberToRemove
			? `Se quitara a ${memberToRemove.username} de este curso.`
			: ""}
		confirmLabel="Quitar"
		isPending={removeMemberMutation.isPending}
		onCancel={() => (memberToRemove = null)}
		onConfirm={confirmRemoveMember}
	/>
</section>
