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
	import { GradeValue } from "$lib/shared/value-objects/grade.value"

	const initialJoinCode = browser
		? (localStorage.getItem("last-submitted-join-code") ?? "")
		: ""

	const resultLookupForm = createForm({
		schema: joinCodeSchema,
		initialInput: { joinCode: initialJoinCode },
	})

	let resultNotice = $state("")
	let noticeTone = $state<"ok" | "warn" | "danger">("warn")

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
			noticeTone = "danger"
			resultNotice = getResultErrorMessage(error)
			toast.error(resultNotice)
		},
	}))

	const byAttemptIdMutation = createMutation(() => ({
		mutationFn: (attemptId: string) => attemptsService.getResultsOrThrow(attemptId),
		onSuccess: data => {
			noticeTone = "ok"
			resultNotice = `Resultado encontrado. Puntaje: ${data.score}/${data.maxScore}, Nota: ${GradeValue.format(data.grade)}.`
			toast.success("Resultado cargado correctamente.")
		},
		onError: error => {
			noticeTone = "danger"
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

<section class="grid gap-5">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Resultados</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Busca tu resultado por código o recupera tu último intento guardado.
		</p>
	</header>

	<section class="panel-elevated p-5 sm:p-6">
		<Form
			of={resultLookupForm}
			onsubmit={submitLookup}
			class="grid gap-3 lg:grid-cols-[3fr_1fr_1fr] lg:items-end"
		>
			<Field of={resultLookupForm} path={["joinCode"]}>
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
				disabled={byJoinCodeMutation.isPending}
			>
				<Search size={16} aria-hidden="true" />
				{byJoinCodeMutation.isPending ? "Consultando..." : "Ir a sala de espera"}
			</button>

			<button
				class="btn-secondary flex h-11 items-center gap-1.5 px-3 text-xs sm:text-sm"
				type="button"
				onclick={loadLastAttempt}
			>
				<History size={16} aria-hidden="true" />
				Cargar último intento
			</button>
		</Form>

		<div class="keyline mt-4"></div>
		<p class="mt-4 mb-0 text-sm text-zinc-600">
			Si el resultado aún no está publicado, te llevaremos a una sala de espera con
			actualización automática.
		</p>

		{#if resultNotice}
			<p
				class="notice mt-4 mb-0"
				class:notice-ok={noticeTone === "ok"}
				class:notice-warn={noticeTone === "warn"}
				class:notice-danger={noticeTone === "danger"}
			>
				{resultNotice}
			</p>
		{/if}
	</section>
</section>
