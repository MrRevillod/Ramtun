<script lang="ts">
	import type { SubmitEventHandler } from "@formisch/svelte"
	import type { LoginDTO, LoginDTOSchema } from "$lib/auth/dtos"

	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { LogIn } from "lucide-svelte"
	import { authStore } from "$lib/auth/store.svelte"
	import { authService } from "$lib/auth/service"
	import { useMutation } from "$lib/shared/http/tanstack"
	import { loginDTOSchema } from "$lib/auth/dtos"
	import { createForm, Field, Form } from "@formisch/svelte"

	const form = createForm({
		schema: loginDTOSchema,
		initialInput: {
			username: "",
			password: "",
		},
	})

	const mutation = useMutation(() => ({
		mutationFn: (payload: LoginDTO) => authService.login(payload),
		onSuccess: async (user) => {
			authStore.setSession(user)
			await goto("/")
		},
		onError: (error) => {
			console.error(error)
			toast.error(error.messageOrDefault, {
				duration: 4000,
			})
		},
	}))

	const handleSubmit: SubmitEventHandler<LoginDTOSchema> = async (data) => {
		await mutation.mutateAsync(data)
	}
</script>

<main class="min-h-dvh px-4 py-8 sm:px-6 sm:py-12">
	<div class="mx-auto flex min-h-[calc(100dvh-4rem)] max-w-5xl items-center justify-center">
		<section class="panel-elevated -mt-10 w-full max-w-xl p-8 sm:p-10">
			<h1 class="m-0 mt-2 text-3xl leading-tight text-black sm:text-[2.15rem]">Iniciar sesión</h1>
			<p class="mt-3 mb-7 max-w-md text-base leading-relaxed text-zinc-700">
				Usa tus credenciales Pillan/LDAP para ingresar.
			</p>

			<Form of={form} onsubmit={handleSubmit} class="grid gap-4">
				<Field of={form} path={["username"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Usuario</span>
							<input
								{...field.props}
								class="input-base"
								type="text"
								value={field.input ?? ""}
								autocomplete="username"
								placeholder="fgutierrez"
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>

				<Field of={form} path={["password"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Contraseña</span>
							<input
								{...field.props}
								class="input-base"
								type="password"
								value={field.input ?? ""}
								autocomplete="off"
								placeholder="•••••••"
							/>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>

				<button
					class="btn-primary mt-2 flex w-full items-center justify-center gap-1.5 text-base"
					type="submit"
					disabled={mutation.isPending}
				>
					<LogIn size={16} aria-hidden="true" />
					{mutation.isPending ? "Ingresando..." : "Ingresar"}
				</button>
			</Form>

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
