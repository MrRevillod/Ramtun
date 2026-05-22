<script lang="ts">
	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
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
	import {
		connectAttemptsSocket,
		disconnectAttemptsSocket,
	} from "$lib/shared/socket/attempts.socket"
	import { themeStore } from "$lib/shared/theme/theme.store.svelte"

	let { children } = $props()

	const logoutMutation = createMutation(() => ({
		mutationFn: () => authService.logoutOrThrow(),
		onSuccess: async () => {
			sessionManager.clearSession()
			await goto("/login")
		},
		onError: error => {
			toast.error(getErrorMessage(error))
			sessionManager.clearSession()
			void goto("/login")
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
	const isAuthenticated = $derived(authStore.isAuthenticated())

	$effect(() => {
		if (!isAuthenticated) {
			disconnectAttemptsSocket()
			return
		}

		connectAttemptsSocket()
		return () => disconnectAttemptsSocket()
	})
</script>

<main
	class="mx-auto flex max-h-dvh min-h-dvh max-w-312 flex-col gap-3 p-4 sm:p-4 lg:p-6"
>
	<header class="panel-elevated mx-auto w-full gap-4 p-4 sm:p-5">
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
					title={themeStore.preference === "dark"
						? "Cambiar a modo claro"
						: "Cambiar a modo oscuro"}
				>
					{#if themeStore.preference === "dark"}
						<Sun size={16} aria-hidden="true" />
					{:else}
						<Moon size={16} aria-hidden="true" />
					{/if}
				</button>

				<div
					class="flex h-11 min-w-0 flex-1 flex-col justify-center rounded-md border border-zinc-200 bg-zinc-50 px-3 text-left sm:min-w-56 sm:flex-none sm:text-right"
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

	<nav class="panel-elevated flex w-full gap-1.5 px-4 py-2">
		<a
			class="action-tab flex-1 justify-center"
			data-active={isActive("/join")}
			href="/join"
		>
			<DoorOpen size={16} aria-hidden="true" />
			Unirse
		</a>
		<a
			class="action-tab flex-1 justify-center"
			data-active={isActive("/results")}
			href="/results"
		>
			<ClipboardList size={16} aria-hidden="true" />
			Resultados
		</a>
		{#if showTeachingNav}
			<a
				class="action-tab flex-1 justify-center"
				data-active={isActive("/courses")}
				href="/courses"
			>
				<Layers size={16} aria-hidden="true" />
				Cursos
			</a>
		{/if}
		{#if showUsersNav}
			<a
				class="action-tab flex-1 justify-center"
				data-active={isActive("/admin/users")}
				href="/admin/users"
			>
				<Users size={16} aria-hidden="true" />
				Usuarios
			</a>
		{/if}
	</nav>

	<section class="panel-elevated mx-auto w-full min-w-0 p-4">
		{@render children()}
	</section>
</main>
