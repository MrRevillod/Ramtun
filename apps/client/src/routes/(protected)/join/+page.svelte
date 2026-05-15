<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { createMutation } from "@tanstack/svelte-query"
	import { createForm, Field, Form, type SubmitEventHandler } from "@formisch/svelte"
	import { Search } from "lucide-svelte"
	import { toast } from "svelte-sonner"
	import {
		joinCodeSchema,
		type JoinCodeFormValues,
	} from "$lib/attempts/attempts.schema"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"

	const joinForm = createForm({
		schema: joinCodeSchema,
		initialInput: { joinCode: "" },
	})

	const joinMutation = createMutation(() => ({
		mutationFn: ({ joinCode }: JoinCodeFormValues) =>
			quizzesService.joinByCodeOrThrow(joinCode),
		onSuccess: async (_preview, input) => {
			await goto(
				resolve(`/join/lobby?joinCode=${encodeURIComponent(input.joinCode)}`)
			)
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const submitJoin: SubmitEventHandler<typeof joinCodeSchema> = async output => {
		await joinMutation.mutateAsync(output)
	}
</script>

<section class="grid gap-5">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Unirse a un quiz</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Ingresa tu código para revisar los datos del quiz antes de comenzar.
		</p>
	</header>

	<section class="panel-elevated p-5 sm:p-6">
		<Form
			of={joinForm}
			onsubmit={submitJoin}
			class="grid gap-3 lg:grid-cols-[3fr_1fr] lg:items-end"
		>
			<Field of={joinForm} path={["joinCode"]}>
				{#snippet children(field)}
					<div class="grid gap-1.5">
						<input
							{...field.props}
							class="input-base"
							aria-label="Código de quiz"
							value={field.input ?? ""}
							placeholder="Ingrese un código de Quiz. Ej: ABC1234"
						/>
						{#if field.errors?.[0]}
							<span class="text-sm text-red-700">{field.errors[0]}</span>
						{/if}
					</div>
				{/snippet}
			</Field>

			<button
				class="btn-primary flex h-11 items-center gap-1.5 px-3 text-xs sm:text-sm"
				type="submit"
				disabled={joinMutation.isPending}
			>
				<Search size={16} aria-hidden="true" />
				{joinMutation.isPending ? "Validando..." : "Entrar"}
			</button>
		</Form>
	</section>
</section>
