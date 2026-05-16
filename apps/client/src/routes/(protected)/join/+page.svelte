<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { createMutation } from "@tanstack/svelte-query"
	import { Search } from "lucide-svelte"
	import { toast } from "svelte-sonner"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { joinCodeSchema } from "$lib/attempts/attempts.dtos"

	let joinCode = $state("")
	let joinError = $state("")

	const joinMutation = createMutation(() => ({
		mutationFn: (code: string) => quizzesService.joinByCodeOrThrow(code),
		onSuccess: async (_preview, code) => {
			await goto(resolve(`/join/lobby?joinCode=${encodeURIComponent(code)}`))
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const handleSubmit = () => {
		joinError = ""
		const result = joinCodeSchema.safeParse({ joinCode })
		if (!result.success) {
			joinError = result.issues[0].message
			return
		}
		void joinMutation.mutateAsync(result.output.joinCode)
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
		<div class="flex flex-col gap-3 sm:flex-row sm:items-end">
			<div class="flex-1">
				<div class="grid gap-1.5">
					<input
						class="input-base"
						type="text"
						aria-label="Código de quiz"
						bind:value={joinCode}
						placeholder="Ingrese un código de Quiz. Ej: ABC1234"
						oninput={() => (joinError = "")}
					/>
					{#if joinError}
						<span class="text-sm text-red-700">{joinError}</span>
					{/if}
				</div>
			</div>

			<button
				class="btn-primary flex h-11 shrink-0 items-center gap-1.5 px-3 text-xs sm:text-sm"
				type="button"
				disabled={joinMutation.isPending}
				onclick={handleSubmit}
			>
				<Search size={16} aria-hidden="true" />
				{joinMutation.isPending ? "Validando..." : "Entrar"}
			</button>
		</div>
	</section>
</section>
