<script lang="ts">
	import type { LayoutProps } from "./$types"

	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { onMount } from "svelte"
	import { authStore } from "$lib/auth/store.svelte"
	import { useMutation, useQuery } from "$lib/shared/http/tanstack"
	import { authService } from "$lib/auth/service"
	import { DoorOpen, Layers, ClipboardList, LogOut } from "lucide-svelte"
	import {
		connectAttemptsSocket,
		disconnectAttemptsSocket,
	} from "$lib/shared/socket/attempts.socket"
	import { attemptsService } from "$lib/attempts/attempts.service"

	let { data, children }: LayoutProps = $props()

	const user = authStore.user
	const showCoursesNav = $derived(user?.isAdmin() || user?.isFunc() || data.hasCourses)

	$effect(() => {
		if (user?.isStudent() && !data.hasCourses && !page.url.pathname.startsWith("/join")) {
			void goto("/join")
		}
	})

	const activeAttemptQuery = useQuery(() => ({
		queryKey: ["attempts", "active"] as const,
		queryFn: () => attemptsService.getActive(),
		retry: false,
		refetchOnMount: false,
		refetchOnWindowFocus: false,
	}))

	$effect(() => {
		const active = activeAttemptQuery.data
		if (!active) return

		const attemptPath = `/attempts/${active.attemptId}`

		if (!page.url.pathname.startsWith(attemptPath)) {
			void goto(attemptPath)
		}
	})

	const logoutMutation = useMutation(() => ({
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

	const isActive = (href: string) => {
		return page.url.pathname.startsWith(href)
	}

	onMount(() => {
		if (authStore.isAuthenticated()) {
			connectAttemptsSocket()
		}

		return () => disconnectAttemptsSocket()
	})
</script>

<main class="mx-auto flex min-h-dvh max-w-312 flex-col gap-2 p-3 sm:p-4 lg:p-6">
	<header class="panel-elevated mx-auto w-full p-3 sm:p-4">
		<div class="flex items-center justify-between gap-4 px-8">
			<div>
				<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">INF-UCT: RAMTUN</p>
				<h1 class="mt-0.5 mb-0 text-xl leading-tight text-black sm:text-2xl">
					Cuestionarios en línea
				</h1>
			</div>

			<div class="mt-2.5 flex items-center gap-1">
				<a class="action-tab justify-center" data-active={isActive("/join")} href="/join">
					<DoorOpen size={16} aria-hidden="true" />
					<span class="hidden lg:inline">Unirse</span>
				</a>
				<a class="action-tab justify-center" data-active={isActive("/results")} href="/results">
					<ClipboardList size={16} aria-hidden="true" />
					<span class="hidden lg:inline">Resultados</span>
				</a>
				{#if showCoursesNav}
					<a class="action-tab justify-center" data-active={isActive("/courses")} href="/courses">
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
						<p class="text-xs text-zinc-600">{authStore.user?.role.toDisplay()}</p>
					</div>
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
