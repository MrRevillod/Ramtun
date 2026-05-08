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
		Moon,
		Sun,
	} from "lucide-svelte"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { canManageUsers, isTeachingRole } from "$lib/shared/auth/permissions"
	import { sessionManager } from "$lib/shared/auth/session.manager"
	import { getErrorMessage } from "$lib/shared/errors"
	import { roleLabel } from "$lib/shared/labels"
	import { themeStore } from "$lib/shared/theme/theme.store.svelte"

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
	<header class="panel-elevated mx-auto w-full max-w-[74rem] p-4 sm:p-5">
		<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
			<div>
				<p class="section-kicker m-0">INF-UCT: RAMTUN</p>
				<h1 class="mt-2 text-2xl leading-tight text-black sm:text-3xl">
					Cuestionarios y Tests de Certeza
				</h1>
				<p class="mt-1 mb-0 text-sm text-zinc-600">
					Desarrollado por y para miembros de la comunidad UCT.
				</p>
			</div>

			<div class="flex w-full items-stretch gap-2 sm:w-auto">
				<button
					class="inline-flex size-11 items-center justify-center self-stretch rounded-md border border-zinc-300 bg-white text-zinc-800 transition-colors duration-200 hover:border-zinc-900 hover:bg-zinc-100 sm:size-10"
					type="button"
					onclick={() => themeStore.toggle()}
					title={themeStore.resolved === "dark"
						? "Cambiar a modo claro"
						: "Cambiar a modo oscuro"}
				>
					{#if themeStore.resolved === "dark"}
						<Sun size={16} aria-hidden="true" />
					{:else}
						<Moon size={16} aria-hidden="true" />
					{/if}
				</button>

				<div
					class="min-w-0 flex-1 rounded-md border border-zinc-200 bg-zinc-50 px-3 text-left sm:min-w-[14rem] sm:flex-none sm:text-right h-11 flex flex-col justify-center"
				>
					<p class="truncate text-sm font-semibold text-zinc-800">
						{authStore.session?.user.name}
					</p>
					<p class="text-xs text-zinc-600">{displayRole}</p>
				</div>
				<button
					class="inline-flex size-11 items-center justify-center self-stretch rounded-md border border-zinc-900 bg-zinc-900 text-white transition-colors duration-200 hover:bg-zinc-800 sm:size-10"
					type="button"
					onclick={handleLogout}
					title="Cerrar sesión"
				>
					<LogOut size={16} aria-hidden="true" />
				</button>
			</div>
		</div>
	</header>

	<section class="mx-auto mt-3 w-full max-w-[74rem] flex-1">
		<nav class="panel-muted grid grid-cols-3 gap-1.5 p-2 lg:grid-cols-3">
			<a
				class="action-tab flex items-center justify-center gap-1.5 whitespace-nowrap"
				data-active={isActive("/join")}
				href={resolve("/join")}
			>
				<DoorOpen size={16} aria-hidden="true" />
				Unirse
			</a>
			<a
				class="action-tab flex items-center justify-center gap-1.5 whitespace-nowrap"
				data-active={isActive("/results")}
				href={resolve("/results")}
			>
				<ClipboardList size={16} aria-hidden="true" />
				Resultados
			</a>
			{#if showTeachingNav}
				<a
					class="action-tab flex items-center justify-center gap-1.5 whitespace-nowrap"
					data-active={isActive("/courses")}
					href={resolve("/courses")}
				>
					<Layers size={16} aria-hidden="true" />
					Cursos
				</a>
			{/if}
			{#if showUsersNav}
				<a
					class="action-tab flex items-center justify-center gap-1.5 whitespace-nowrap"
					data-active={isActive("/admin/users")}
					href={resolve("/admin/users")}
				>
					<Users size={16} aria-hidden="true" />
					Usuarios
				</a>
			{/if}
		</nav>

		<section class="panel-elevated mt-3 min-w-0 p-4 sm:p-5 lg:p-6">
			{@render children()}
		</section>
	</section>
</main>
