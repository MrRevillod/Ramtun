<script lang="ts">
	import type { LoginInput } from "$lib/auth/types"

	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { LogIn } from "lucide-svelte"
	import { authService } from "$lib/auth/auth.service"
	import { sessionManager } from "$lib/shared/auth/session.manager"
	import { getErrorMessage } from "$lib/shared/errors"

	let username = $state("")
	let password = $state("")

	const loginMutation = createMutation(() => ({
		mutationFn: (payload: LoginInput) => authService.loginOrThrow(payload),
		onSuccess: async session => {
			sessionManager.setSession(session)
			await goto(resolve("/"))
		},
		onError: error => {
			toast.error(getErrorMessage(error))
		},
	}))

	const loading = $derived(loginMutation.isPending)

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault()
		await loginMutation.mutateAsync({ username, password })
	}
</script>

<main class="min-h-dvh px-4 py-8 sm:px-6 sm:py-12">
	<div
		class="mx-auto flex min-h-[calc(100dvh-4rem)] max-w-5xl items-center justify-center"
	>
		<section class="panel-elevated w-full max-w-xl p-8 sm:p-10">
			<h1 class="m-0 mt-2 text-3xl leading-tight text-black sm:text-[2.15rem]">
				Iniciar sesión
			</h1>
			<p class="mt-3 mb-7 max-w-md text-base leading-relaxed text-zinc-700">
				Usa tus credenciales Pillan/LDAP para ingresar.
			</p>

			<form class="grid gap-4" onsubmit={handleSubmit}>
				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Usuario</span>
					<input
						class="input-base"
						type="text"
						bind:value={username}
						required
						autocomplete="username"
					/>
				</label>

				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Contraseña</span>
					<input
						class="input-base"
						type="password"
						bind:value={password}
						required
						autocomplete="current-password"
					/>
				</label>

				<button
					class="btn-primary mt-2 flex w-full items-center justify-center gap-1.5 text-base"
					type="submit"
					disabled={loading}
				>
					<LogIn size={16} aria-hidden="true" />
					{loading ? "Ingresando..." : "Ingresar"}
				</button>
			</form>

			<p class="mt-5 mb-0 text-sm text-zinc-700">
				<a
					class="font-medium text-zinc-900 underline decoration-zinc-400 underline-offset-3 hover:decoration-zinc-900"
					href="https://chpass.inf.uct.cl"
					target="_blank"
					rel="noopener noreferrer"
				>
					¿Olvidaste tu contraseña?
				</a>
			</p>
		</section>
	</div>
</main>
