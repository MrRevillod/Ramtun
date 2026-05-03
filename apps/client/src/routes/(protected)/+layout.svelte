<script lang="ts">
	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { sessionManager } from "$lib/shared/auth/session.manager"
	import { getErrorMessage } from "$lib/shared/errors"

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

	const roleLabel = $derived.by(() => {
		const role = authStore.session?.user.role

		if (role === "func") {
			return "Profesor"
		}

		if (role === "assistant") {
			return "Ayudante"
		}

		return "Estudiante"
	})
</script>

<main class="app-shell">
	<header
		class="panel-surface flex flex-col gap-4 p-4 sm:flex-row sm:items-center sm:justify-between sm:p-5"
	>
		<div>
			<h1 class="text-2xl leading-tight text-black sm:text-3xl">
				Cuestionarios y Tests de Certeza
			</h1>
		</div>

		<div class="flex flex-col gap-3 sm:flex-row sm:items-center">
			<div
				class="rounded-[4px] border border-zinc-200 bg-zinc-50 px-3 py-2 text-left sm:text-right"
			>
				<p class="text-sm font-semibold text-zinc-800">
					{authStore.session?.user.name}
				</p>
				<p class="text-xs text-zinc-600">{roleLabel}</p>
			</div>
			<button
				class="btn-primary w-full sm:w-auto"
				type="button"
				onclick={handleLogout}
			>
				Salir
			</button>
		</div>
	</header>

	<section class="panel-surface mt-3 flex-1 p-4 sm:p-5 lg:p-6">
		{@render children()}
	</section>
</main>
