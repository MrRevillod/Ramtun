<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { createMutation } from "@tanstack/svelte-query"
	import { browser } from "$app/environment"
	import { toast } from "svelte-sonner"
	import { Search, History } from "lucide-svelte"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import { joinCodeFormSchema, type JoinCodeFormData } from "$lib/attempts/attempts.dtos"
	import { getErrorMessage } from "$lib/shared/errors"
	import { createForm, Field, Form } from "@formisch/svelte"

	const form = createForm({
		schema: joinCodeFormSchema,
		validate: "blur",
		initialInput: {
			joinCode: browser
				? (localStorage.getItem("last-submitted-join-code") ?? "")
				: "",
		},
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
			attemptsService.getResultsByJoinCodeOrThrow(joinCode),
		onSuccess: async (_data, joinCode) => {
			await goto(resolve(`/results/lobby?joinCode=${encodeURIComponent(joinCode)}`))
		},
		onError: error => {
			noticeTone = "danger"
			resultNotice = getResultErrorMessage(error)
			toast.error(resultNotice)
		},
	}))

	const loadLastAttempt = async () => {
		if (!browser) return
		const code = localStorage.getItem("last-submitted-join-code")
		if (!code) {
			toast.error("No hay un intento reciente guardado.")
			return
		}

		await byJoinCodeMutation.mutateAsync(code)
	}

	const handleSubmit = (input: JoinCodeFormData) => {
		if (browser) {
			localStorage.setItem("last-submitted-join-code", input.joinCode)
		}
		byJoinCodeMutation.mutateAsync(input.joinCode)
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
		<Form of={form} onsubmit={handleSubmit}>
			<div class="flex flex-col gap-3 sm:flex-row sm:items-start">
				<div class="flex-1">
					<Field of={form} path={["joinCode"]}>
						{#snippet children(field)}
							<label class="grid gap-1.5">
								<input
									class="input-base"
									type="text"
									aria-label="Código de quiz"
									{...field.props}
									value={field.input}
									placeholder="Ingrese un código de Quiz. Ej: ABC1234"
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
					class="btn-primary flex h-11 shrink-0 items-center gap-1.5 px-3 text-xs sm:text-sm"
					type="submit"
					disabled={byJoinCodeMutation.isPending}
				>
					<Search size={16} aria-hidden="true" />
					{byJoinCodeMutation.isPending ? "Consultando..." : "Ir a sala de espera"}
				</button>

				<button
					class="btn-secondary flex h-11 shrink-0 items-center gap-1.5 px-3 text-xs sm:text-sm"
					type="button"
					onclick={loadLastAttempt}
				>
					<History size={16} aria-hidden="true" />
					Cargar último intento
				</button>
			</div>
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
