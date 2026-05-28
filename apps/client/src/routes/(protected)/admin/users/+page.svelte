<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { toast } from "svelte-sonner"
	import { Search, ArrowUpDown } from "lucide-svelte"
	import type { ManagedUser, ManagedUserRole } from "$lib/users/users.dtos"
	import { usersService } from "$lib/users/users.service"
	import { isFunc, isAdmin } from "$lib/shared/auth/permissions"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { getErrorMessage } from "$lib/shared/errors"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"

	const queryClient = useQueryClient()
	const usersKey = ["admin", "users"]

	let search = $state("")
	let roleFilter = $state("")
	let userToEdit = $state<{
		id: string
		name: string
		newRole: ManagedUserRole
	} | null>(null)

	const usersQuery = createQuery(() => ({
		queryKey: [...usersKey, { search, roleFilter }],
		queryFn: async () => {
			const params: { search?: string; roles?: string } = {}
			if (search.trim()) params.search = search.trim()
			if (roleFilter) params.roles = roleFilter
			return await usersService.listUsers(params)
		},
	}))

	const updateRoleMutation = createMutation(() => ({
		mutationFn: ({ userId, role }: { userId: string; role: ManagedUserRole }) =>
			usersService.updateRole(userId, role),
		onSuccess: async () => {
			toast.success("Rol actualizado correctamente.")
			userToEdit = null
			await queryClient.invalidateQueries({ queryKey: usersKey })
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const confirmRoleChange = async () => {
		if (!userToEdit) return
		await updateRoleMutation.mutateAsync({
			userId: userToEdit.id,
			role: userToEdit.newRole,
		})
	}

	const currentRole = $derived(authStore.session?.user.role)
	const canEditRole = $derived(
		(r: ManagedUser["role"]) =>
			isAdmin(currentRole) || (isFunc(currentRole) && r === "student")
	)
</script>

<section class="grid gap-4">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Usuarios y roles</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Consulta cuentas institucionales y ajusta permisos de acceso por rol.
		</p>
	</header>

	<section class="panel-elevated p-4 sm:p-5">
		<div class="mb-4 flex flex-wrap items-center gap-3">
			<div class="relative min-w-0 flex-1">
				<Search
					size={16}
					class="pointer-events-none absolute top-1/2 left-3 -translate-y-1/2 text-zinc-400"
					aria-hidden="true"
				/>
				<input
					type="text"
					placeholder="Buscar por nombre o usuario..."
					bind:value={search}
					class="w-full rounded-md border border-zinc-300 py-2 pr-3 pl-9 text-sm text-zinc-900 placeholder-zinc-400"
				/>
			</div>
			<select
				bind:value={roleFilter}
				class="cursor-pointer rounded-md border border-zinc-300 bg-white px-3 py-2 text-sm text-zinc-700"
			>
				<option value="">Todos los roles</option>
				<option value="student">Estudiante</option>
				<option value="func">Profesor</option>
				<option value="admin">Administrador</option>
			</select>
		</div>

		{#if usersQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando usuarios...</p>
		{:else if usersQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(usersQuery.error)}</p>
		{:else if !usersQuery.data?.length}
			<p class="notice notice-warn m-0">
				{search || roleFilter
					? "No se encontraron usuarios con los filtros actuales."
					: "No hay usuarios registrados."}
			</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Usuario</th>
							<th class="px-3 py-2 text-left font-medium">Email</th>
							<th class="px-3 py-2 text-left font-medium">Rol</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each usersQuery.data as user (user.id)}
							<tr class="table-row">
								<td class="px-3 py-2 text-zinc-800">{user.name}</td>
								<td class="px-3 py-2 font-medium text-zinc-900">{user.username}</td>
								<td class="px-3 py-2 text-zinc-600">{user.email}</td>
								<td class="px-3 py-2">
									<span
										class="inline-block rounded-full px-2.5 py-0.5 text-xs font-medium"
										class:bg-amber-100={user.role === "student"}
										class:text-amber-800={user.role === "student"}
										class:bg-blue-100={user.role === "func"}
										class:text-blue-800={user.role === "func"}
										class:bg-purple-100={user.role === "admin"}
										class:text-purple-800={user.role === "admin"}
									>
										{user.role === "student" && "Estudiante"}
										{user.role === "func" && "Profesor"}
										{user.role === "admin" && "Administrador"}
									</span>
								</td>
								<td class="px-3 py-2">
									{#if canEditRole(user.role)}
										<div class="flex items-center gap-1">
											<button
												class="icon-btn cursor-pointer"
												title="Cambiar rol"
												type="button"
												onclick={() => {
													const newRole: ManagedUserRole =
														user.role === "student" ? "func" : "student"
													userToEdit = { id: user.id, name: user.name, newRole }
												}}
											>
												<ArrowUpDown size={15} aria-hidden="true" />
											</button>
										</div>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>

<ConfirmActionModal
	open={!!userToEdit}
	title="Cambiar rol"
	message={userToEdit
		? `Se cambiara el rol de ${userToEdit.name} a ${userToEdit.newRole === "func" ? "Profesor" : "Estudiante"}.`
		: ""}
	confirmLabel="Cambiar"
	isPending={updateRoleMutation.isPending}
	onCancel={() => (userToEdit = null)}
	onConfirm={confirmRoleChange}
/>
