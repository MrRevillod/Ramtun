<script lang="ts">
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { Search } from "lucide-svelte"
	import { createMutation } from "@tanstack/svelte-query"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import {
		joinCodeFormSchema,
		type JoinCodeFormData,
	} from "$lib/attempts/attempts.dtos"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	const form = createForm({
		schema: joinCodeFormSchema,
		validate: "blur",
	})

	const joinMutation = createMutation(() => ({
		mutationFn: (code: string) => quizzesService.joinByCode(code),
		onSuccess: async (_, code) => {
			reset(form)
			await goto(`/join/lobby?joinCode=${encodeURIComponent(code)}`)
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const handleSubmit = (input: JoinCodeFormData) => {
		joinMutation.mutateAsync(input.joinCode)
	}
</script>

<section class="flex flex-col gap-4">
	<div>
		<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">
			Acceso rápido
		</p>
		<h2 class="mt-0.5 mb-0 text-2xl tracking-tight text-black">
			Unirse a un cuestionario
		</h2>
	</div>

	<section>
		<Form of={form} onsubmit={handleSubmit}>
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start">
				<div class="flex-1">
					<Field of={form} path={["joinCode"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<input
									class="input-base"
									type="text"
									aria-label="Código de cuestionario"
									{...field.props}
									value={field.input}
									placeholder="Ingrese un código del cuestionario. Ej: ABC1234"
								/>
								<span
									class="text-sm text-red-700"
									class:invisible={!field.errors?.[0]}
									aria-live="polite"
								>
									{field.errors?.[0] ?? " "}
								</span>
							</label>
						{/snippet}
					</Field>
				</div>

				<button
					class="btn-primary flex h-11 shrink-0 cursor-pointer items-center gap-1.5 px-3 text-xs sm:text-sm"
					type="submit"
					disabled={joinMutation.isPending}
				>
					<Search size={16} aria-hidden="true" />
					{joinMutation.isPending ? "Validando..." : "Entrar"}
				</button>
			</div>
		</Form>
	</section>
</section>
