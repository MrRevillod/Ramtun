<script lang="ts">
	import { goto } from "$app/navigation"
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
			await goto(`/join/lobby?joinCode=${encodeURIComponent(input.joinCode)}`)
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const submitJoin: SubmitEventHandler<typeof joinCodeSchema> = async output => {
		await joinMutation.mutateAsync(output)
	}
</script>

<section class="grid gap-4">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Unirse a quiz</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Ingresa el codigo de acceso para ir al lobby del quiz.
		</p>
	</header>

	<section class="panel-muted p-4 sm:p-5">
		<h3 class="m-0 text-lg text-black">Codigo de acceso</h3>
		<Form
			of={joinForm}
			onsubmit={submitJoin}
			class="mt-3 flex flex-col gap-3 sm:flex-row sm:items-end"
		>
			<Field of={joinForm} path={["joinCode"]}>
				{#snippet children(field)}
					<label class="grid flex-1 gap-1.5">
						<span class="text-sm text-zinc-800">Join code</span>
						<input
							{...field.props}
							class="input-base"
							value={field.input ?? ""}
							placeholder="Ej: ABCD1234"
						/>
						{#if field.errors?.[0]}
							<span class="text-sm text-red-700">{field.errors[0]}</span>
						{/if}
					</label>
				{/snippet}
			</Field>

			<button
				class="btn-primary flex items-center gap-1.5"
				type="submit"
				disabled={joinMutation.isPending}
			>
				<Search size={16} aria-hidden="true" />
				{joinMutation.isPending ? "Validando..." : "Ir al lobby"}
			</button>
		</Form>
	</section>
</section>
