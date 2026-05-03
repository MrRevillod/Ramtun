<script lang="ts">
	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import {
		DoorOpen,
		Layers,
		Users,
		ClipboardList,
		LogOut,
		User,
	} from "lucide-svelte"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { canManageUsers, isTeachingRole } from "$lib/shared/auth/permissions"
	import { sessionManager } from "$lib/shared/auth/session.manager"
	import { getErrorMessage } from "$lib/shared/errors"
	import { roleLabel } from "$lib/shared/labels"

	let { children } = $props()

	const logoutMutation = createMutation(() => ({
		mutationFn: () => authService.logoutOrThrow(),
		onSuccess: async () => {
			sessionManager.clearSession()
			await goto(resolve("/login"))
		},
		onError: error => {
			toast.error(getErrorMessage(error))
			sessionManager.clearSession()
			void goto(resolve("/login"))
		},
	}))

	const handleLogout = async () => {
		await logoutMutation.mutateAsync()
	}

	const role = $derived(authStore.session?.user.role)
	const displayRole = $derived(role ? roleLabel(role) : "")
	const showTeachingNav = $derived(isTeachingRole(role))
	const showUsersNav = $derived(canManageUsers(role))
	const isActive = (href: string) => page.url.pathname.startsWith(href)
</script>

<main class="app-shell">
	<header
		class="panel-surface relative overflow-hidden p-4 sm:p-5"
	>
		<div class="pointer-events-none absolute inset-x-0 top-0 h-16 bg-gradient-to-r from-amber-100/35 via-emerald-100/20 to-transparent"></div>
		<div class="relative flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
		<div>
			<p class="section-kicker m-0">Plataforma academica</p>
			<h1 class="mt-2 text-2xl leading-tight text-black sm:text-3xl">
				Cuestionarios y Tests de Certeza
			</h1>
		</div>

		<div class="flex flex-col gap-3 sm:flex-row sm:items-center">
			<div class="flex items-center gap-2 rounded-md border border-zinc-200 bg-zinc-50 px-3 py-2 text-left sm:text-right">
				<User size={16} class="shrink-0 text-zinc-500" aria-hidden="true" />
				<div>
					<p class="text-sm font-semibold text-zinc-800">
						{authStore.session?.user.name}
					</p>
					<p class="text-xs text-zinc-600">{displayRole}</p>
				</div>
			</div>
			<button
				class="btn-primary flex w-full items-center gap-1.5 sm:w-auto"
				type="button"
				onclick={handleLogout}
			>
				<LogOut size={16} aria-hidden="true" />
				Salir
			</button>
		</div>
		</div>
	</header>

	<nav class="panel-muted mt-3 flex flex-wrap gap-2 p-2.5">
		<a class="action-tab flex items-center gap-1.5" data-active={isActive("/join")} href={resolve("/join")}>
			<DoorOpen size={16} aria-hidden="true" />
			Unirse
		</a>
		<a class="action-tab flex items-center gap-1.5" data-active={isActive("/results")} href={resolve("/results")}>
			<ClipboardList size={16} aria-hidden="true" />
			Resultados
		</a>
		{#if showTeachingNav}
			<a class="action-tab flex items-center gap-1.5" data-active={isActive("/courses")} href={resolve("/courses")}>
				<Layers size={16} aria-hidden="true" />
				Cursos
			</a>
		{/if}
		{#if showUsersNav}
			<a class="action-tab flex items-center gap-1.5" data-active={isActive("/admin/users")} href={resolve("/admin/users")}>
				<Users size={16} aria-hidden="true" />
				Usuarios
			</a>
		{/if}
	</nav>

	<section class="panel-surface mt-3 flex-1 p-4 sm:p-5 lg:p-6">
		{@render children()}
	</section>
</main>
