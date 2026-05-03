<script lang="ts">
	import { goto } from "$app/navigation"
	import { createMutation } from "@tanstack/svelte-query"
	import { createForm, Field, Form, type SubmitEventHandler } from "@formisch/svelte"
	import { browser } from "$app/environment"
	import { toast } from "svelte-sonner"
	import { Search, History } from "lucide-svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { joinCodeSchema } from "$lib/attempts/attempts.schema"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"

	const initialJoinCode = browser
		? (localStorage.getItem("last-submitted-join-code") ?? "")
		: ""

	const resultLookupForm = createForm({
		schema: joinCodeSchema,
		initialInput: { joinCode: initialJoinCode },
	})

	let resultNotice = $state("")

	const getResultErrorMessage = (error: unknown): string => {
		if (!error || typeof error !== "object") {
			return getErrorMessage(error)
		}

		const maybe = error as { kind?: string; status?: number; message?: string }
		const normalized =
			typeof maybe.message === "string" ? maybe.message.toLowerCase() : ""

		if (
			maybe.kind === "http" &&
			maybe.status === 409 &&
			normalized.includes("not published")
		) {
			return "El docente aun no publica los resultados de este quiz."
		}

		if (maybe.kind === "http" && maybe.status === 404) {
			return "No encontramos un intento para ese codigo en tu cuenta."
		}

		if (maybe.kind === "http" && maybe.status === 403) {
			if (normalized.includes("not published")) {
				return "El docente aun no publica los resultados de este quiz."
			}

			return "No tienes acceso a esos resultados con tu cuenta actual."
		}

		return getErrorMessage(error)
	}

	const byJoinCodeMutation = createMutation(() => ({
		mutationFn: (joinCode: string) =>
			quizzesService.getMyResultByCodeOrThrow(joinCode),
		onSuccess: async (_data, joinCode) => {
			await goto(`/results/lobby?joinCode=${encodeURIComponent(joinCode)}`)
		},
		onError: error => {
			resultNotice = getResultErrorMessage(error)
			toast.error(resultNotice)
		},
	}))

	const byAttemptIdMutation = createMutation(() => ({
		mutationFn: (attemptId: string) => attemptsService.getResultsOrThrow(attemptId),
		onSuccess: data => {
			resultNotice = `Resultado encontrado. Puntaje: ${data.score}/${data.maxScore}, Nota: ${data.grade.toFixed(2)}.`
			toast.success("Resultado cargado correctamente.")
		},
		onError: error => {
			resultNotice = getResultErrorMessage(error)
			toast.error(resultNotice)
		},
	}))

	const submitLookup: SubmitEventHandler<typeof joinCodeSchema> = async output => {
		if (browser) {
			localStorage.setItem("last-submitted-join-code", output.joinCode)
		}
		await byJoinCodeMutation.mutateAsync(output.joinCode)
	}

	const loadLastAttempt = async () => {
		if (!browser) return
		const attemptId = localStorage.getItem("last-submitted-attempt-id")
		if (!attemptId) {
			toast.error("No hay un intento reciente guardado.")
			return
		}

		await byAttemptIdMutation.mutateAsync(attemptId)
	}
</script>

<section class="grid gap-4">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Resultados</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Consulta tu resultado usando el codigo del quiz o carga tu ultimo intento
			enviado.
		</p>
	</header>

	<section class="panel-muted p-4 sm:p-5">
		<div class="mb-3 flex flex-wrap items-center justify-between gap-3">
			<h3 class="m-0 text-lg text-black">Buscar resultado</h3>
			<button
				class="btn-secondary flex items-center gap-1.5"
				type="button"
				onclick={loadLastAttempt}
			>
				<History size={16} aria-hidden="true" />
				Cargar ultimo intento
			</button>
		</div>

		<Form
			of={resultLookupForm}
			onsubmit={submitLookup}
			class="flex flex-col gap-3 sm:flex-row sm:items-end"
		>
			<Field of={resultLookupForm} path={["joinCode"]}>
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
				disabled={byJoinCodeMutation.isPending}
			>
				<Search size={16} aria-hidden="true" />
				{byJoinCodeMutation.isPending ? "Buscando..." : "Buscar"}
			</button>
		</Form>

		{#if resultNotice}
			<p
				class="mt-3 mb-0 rounded-[4px] border border-zinc-300 bg-zinc-100 px-3 py-2 text-sm text-zinc-700"
			>
				{resultNotice}
			</p>
		{/if}
	</section>
</section>
