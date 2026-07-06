<script lang="ts">
	import * as v from "valibot"
	import { loginSchema, type LoginInput } from "$lib/auth/auth.dtos"

	import { createMutation } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { LogIn } from "lucide-svelte"
	import { authService } from "$lib/auth/auth.service"
	import { authStore } from "$lib/auth/auth.store.svelte"
	import { getErrorMessage } from "$lib/shared/errors"

	let username = $state("")
	let password = $state("")
	let errors = $state<Record<string, string>>({})

	const loginMutation = createMutation(() => ({
		mutationFn: (payload: LoginInput) => authService.login(payload),
		onSuccess: async user => {
			authStore.setSession(user)
			password = ""
			await goto(resolve("/"))
		},
		onError: error => {
			toast.error(getErrorMessage(error))
		},
	}))

	const loading = $derived(loginMutation.isPending)

	const handleSubmit = async (event: SubmitEvent) => {
		event.preventDefault()
		errors = {}

		const result = v.safeParse(loginSchema, { username, password })

		if (!result.success) {
			const flat = v.flatten(result.issues)
			if (flat.nested) {
				for (const [key, msgs] of Object.entries(flat.nested)) {
					if (msgs?.length) errors[key] = msgs[0]
				}
			}
			return
		}

		await loginMutation.mutateAsync(result.output)
	}
</script>

<main class="min-h-dvh px-4 py-8 sm:px-6 sm:py-12">
	<div
		class="mx-auto flex min-h-[calc(100dvh-4rem)] max-w-5xl items-center justify-center"
	>
		<section class="panel-elevated -mt-10 w-full max-w-xl p-8 sm:p-10">
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
						class:border-red-500={errors.username}
						type="text"
						bind:value={username}
						autocomplete="username"
					/>
					{#if errors.username}
						<p class="text-xs text-red-600">{errors.username}</p>
					{/if}
				</label>

				<label class="grid gap-1.5">
					<span class="text-sm text-zinc-800">Contraseña</span>
					<input
						class="input-base"
						class:border-red-500={errors.password}
						type="password"
						bind:value={password}
						autocomplete="current-password"
					/>
					{#if errors.password}
						<p class="text-xs text-red-600">{errors.password}</p>
					{/if}
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
