<script lang="ts">
	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { DoorOpen, Layers, ClipboardList, LogOut, Moon, Sun } from "lucide-svelte"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { isAdmin, isFunc } from "$lib/shared/auth/permissions"
	import { coursesService } from "$lib/courses/courses.service"
	import {
		connectAttemptsSocket,
		disconnectAttemptsSocket,
	} from "$lib/shared/socket/attempts.socket"
	import { themeStore } from "$lib/shared/theme.store.svelte"
	import { RoleValue } from "$lib/shared/value-objects/role.value"
	import { onMount } from "svelte"
	import type { AttemptSession } from "$lib/attempts/attempts.dtos"

	let { children } = $props()

	const logoutMutation = createMutation(() => ({
		mutationFn: () => authService.logout(),
		onSettled: async () => {
			authStore.clearSession()
			toast.info("Cerrando sesión...")
			await goto("/login")
		},
	}))

	const handleLogout = async () => {
		await logoutMutation.mutateAsync()
	}

	const role = $derived(authStore.user?.role)
	const displayRole = $derived(role ? RoleValue.format(role) : "")

	let studentHasCourses = $state(false)

	$effect(() => {
		if (role !== "student") {
			studentHasCourses = false
			return
		}

		coursesService
			.list()
			.then(courses => {
				studentHasCourses = courses.length > 0
			})
			.catch(() => {
				studentHasCourses = false
			})
	})

	const showCoursesNav = $derived(isFunc(role) || isAdmin(role) || studentHasCourses)
	const isActive = (href: string) => page.url.pathname.startsWith(href)
	const isAuthenticated = $derived(authStore.isAuthenticated())

	onMount(() => {
		const raw = localStorage.getItem("last-attempt-session")
		if (raw) {
			try {
				const session = JSON.parse(raw) as AttemptSession
				const expiresAt = new Date(session.attempt.expiresAt).getTime()
				const now = Date.now()

				if (session.attempt.submittedAt || expiresAt < now - 60000) {
					localStorage.removeItem("last-attempt-session")
				} else {
					const attemptPath = `/attempts/${session.attempt.attemptId}`
					if (!page.url.pathname.startsWith(attemptPath)) {
						void goto(attemptPath)
						return
					}
				}
			} catch {
				localStorage.removeItem("last-attempt-session")
			}
		}

		if (isAuthenticated) {
			connectAttemptsSocket()
		}

		return () => disconnectAttemptsSocket()
	})
</script>

<main class="mx-auto flex min-h-dvh max-w-312 flex-col gap-2 p-3 sm:p-4 lg:p-6">
	<header class="panel-elevated mx-auto w-full p-3 sm:p-4">
		<div class="flex items-center justify-between gap-4 px-8">
			<div>
				<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">
					INF-UCT: RAMTUN
				</p>
				<h1 class="mt-0.5 mb-0 text-xl leading-tight text-black sm:text-2xl">
					Cuestionarios en línea
				</h1>
			</div>

			<div class="mt-2.5 flex items-center gap-1">
				<a
					class="action-tab justify-center"
					data-active={isActive("/join")}
					href="/join"
				>
					<DoorOpen size={16} aria-hidden="true" />
					<span class="hidden lg:inline">Unirse</span>
				</a>
				<a
					class="action-tab justify-center"
					data-active={isActive("/results")}
					href="/results"
				>
					<ClipboardList size={16} aria-hidden="true" />
					<span class="hidden lg:inline">Resultados</span>
				</a>
				{#if showCoursesNav}
					<a
						class="action-tab justify-center"
						data-active={isActive("/courses")}
						href="/courses"
					>
						<Layers size={16} aria-hidden="true" />
						<span class="hidden lg:inline">Cursos</span>
					</a>
				{/if}

				<span class="mx-1 h-6 w-px bg-zinc-300" aria-hidden="true"></span>

				<div class="flex shrink-0 items-stretch gap-1.5 sm:gap-2">
					<div
						class="flex h-9 min-w-0 flex-col justify-center rounded-md border border-zinc-200 bg-zinc-50 px-2.5 text-left sm:h-10 sm:min-w-44 sm:px-3 sm:text-right"
					>
						<p class="truncate text-sm font-semibold text-zinc-800">
							{authStore.user?.name}
						</p>
						<p class="text-xs text-zinc-600">{displayRole}</p>
					</div>
					<button
						class="inline-flex size-9 cursor-pointer items-center justify-center self-stretch rounded-md border border-zinc-300 bg-white text-zinc-800 transition-colors duration-200 hover:border-zinc-900 hover:bg-zinc-100 sm:size-10"
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
					<button
						class="inline-flex size-9 cursor-pointer items-center justify-center self-stretch rounded-md border border-zinc-900 bg-zinc-900 text-white transition-colors duration-200 hover:bg-zinc-800 sm:size-10"
						type="button"
						onclick={handleLogout}
						title="Cerrar sesión"
					>
						<LogOut size={14} aria-hidden="true" />
					</button>
				</div>
			</div>
		</div>
	</header>

	<section class="panel-elevated mx-auto w-full min-w-0 px-12 py-4">
		{@render children()}
	</section>
</main>
