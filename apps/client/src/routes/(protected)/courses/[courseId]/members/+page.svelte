<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { toast } from "svelte-sonner"
	import { Plus, Trash2 } from "lucide-svelte"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { usersService } from "$lib/users/users.service"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"
	import AddMemberModal from "$lib/courses/components/AddMemberModal.svelte"
	import { RoleValue } from "$lib/shared/value-objects/role.value.js"

	let { data } = $props()

	const currentUserId = $derived(authStore.session?.user.id)

	const queryClient = useQueryClient()

	const membersQuery = createQuery(() => ({
		queryKey: ["course-members", data.courseId],
		queryFn: () => coursesService.listMembers(data.courseId),
	}))

	const candidatesQuery = createQuery(() => ({
		queryKey: ["collaborator-candidates", data.courseId],
		queryFn: () => usersService.listCollaboratorCandidates(),
	}))

	const addMemberMutation = createMutation(() => ({
		mutationFn: (input: { userId: string }) =>
			coursesService.addMember(data.courseId, input),
		onSuccess: async () => {
			toast.success("Miembro agregado correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["course-members", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const removeMemberMutation = createMutation(() => ({
		mutationFn: (userId: string) =>
			coursesService.removeMember(data.courseId, userId),
		onSuccess: async () => {
			toast.success("Miembro removido correctamente.")
			await queryClient.invalidateQueries({
				queryKey: ["course-members", data.courseId],
			})
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

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
				<h3 class="mt-2 mb-0 text-xl text-black">Miembros</h3>
				<p class="mt-2 mb-0 text-zinc-700">Agrega o retira participantes.</p>
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

	<section>
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
								<td class="px-3 py-2 text-zinc-700"
									>{RoleValue.format(member.role)}</td
								>
								<td class="px-3 py-2">
									{#if member.userId !== currentUserId}
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
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>

	<AddMemberModal
		open={showAddModal}
		candidates={candidatesQuery.data ?? []}
		oncancel={() => (showAddModal = false)}
		mutation={addMemberMutation}
	/>

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
