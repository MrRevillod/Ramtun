<script lang="ts">
	import type { JoinCodeFormData } from "$lib/attempts/attempts.dtos"

	import { goto } from "$app/navigation"
	import { page } from "$app/state"
	import { toast } from "svelte-sonner"
	import { Search } from "lucide-svelte"
	import { onMount } from "svelte"
	import { createMutation } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { joinCodeFormSchema } from "$lib/attempts/attempts.dtos"
	import { ApiResponse } from "$lib/shared/http/response"

	const form = createForm({
		schema: joinCodeFormSchema,
		validate: "blur",
	})

	const byJoinCodeMutation = createMutation(() => ({
		mutationFn: (joinCode: string) => attemptsService.getResultsByJoinCode(joinCode),
		onSuccess: async (results, joinCode) => {
			reset(form)
			await goto(`/results/view?joinCode=${encodeURIComponent(joinCode)}`, {
				state: { results },
			})
		},
		onError: error => {
			console.error(error)
			toast.error(ApiResponse.messageOrDefault(error), {
				duration: 4000,
			})
		},
	}))

	const handleSubmit = (input: JoinCodeFormData) => {
		byJoinCodeMutation.mutate(input.joinCode)
	}

	const initialCode = $derived(page.url.searchParams.get("code") ?? "")

	onMount(() => {
		if (initialCode) {
			byJoinCodeMutation.mutate(initialCode)
		}
	})
</script>

<section class="grid gap-5">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Resultados</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">Busca tu resultado por código.</p>
	</header>

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
					disabled={byJoinCodeMutation.isPending}
				>
					<Search size={16} aria-hidden="true" />
					{byJoinCodeMutation.isPending ? "Consultando..." : "Ver resultados"}
				</button>
			</div>
		</Form>
	</section>
</section>
