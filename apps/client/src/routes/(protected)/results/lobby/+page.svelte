<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { createQuery, useQueryClient } from "@tanstack/svelte-query"
	import { browser } from "$app/environment"
	import { onMount } from "svelte"
	import { Loader2, RefreshCw, ArrowLeft } from "lucide-svelte"
	import type { AttemptResult } from "$lib/attempts/types"
	import { quizzesService } from "$lib/quizzes/quizzes.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const joinCode = $derived(data.joinCode as string | undefined)
	const attemptId = $derived(data.attemptId as string | undefined)

	let hasResult = $state(false)
	let result = $state<AttemptResult | null>(null)
	let lastCheckedAt = $state<Date | null>(null)
	let nextCheckIn = $state(15)
	let isChecking = $state(false)

	let countdownTimer: ReturnType<typeof setInterval> | null = null

	const resultQuery = createQuery(() => ({
		queryKey: ["lobby-result", joinCode],
		queryFn: async (): Promise<AttemptResult | null> => {
			if (!joinCode) return null
			try {
				const r = await quizzesService.getMyResultByCodeOrThrow(joinCode)
				return r
			} catch (e) {
				const maybe = e as { kind?: string; status?: number }
				if (
					maybe.kind === "http" &&
					maybe.status === 409
				) {
					return null
				}
				throw e
			}
		},
		refetchInterval: hasResult ? false : 15000,
		enabled: !!joinCode && !hasResult,
	}))

	$effect(() => {
		if (resultQuery.data) {
			result = resultQuery.data
			hasResult = true
		}
	})

	onMount(() => {
		countdownTimer = setInterval(() => {
			nextCheckIn = nextCheckIn > 0 ? nextCheckIn - 1 : 15
		}, 1000)

		return () => {
			if (countdownTimer) clearInterval(countdownTimer)
		}
	})

	const checkNow = async () => {
		if (isChecking) return
		isChecking = true
		lastCheckedAt = new Date()
		nextCheckIn = 15

		try {
			const r = await quizzesService.getMyResultByCodeOrThrow(joinCode!)
			result = r
			hasResult = true
		} catch (e) {
			const maybe = e as { kind?: string; status?: number }
			if (maybe.kind !== "http" || maybe.status !== 409) {
				lastCheckedAt = new Date()
			}
		} finally {
			isChecking = false
		}
	}

	const goBack = async () => {
		await goto(resolve("/join"))
	}
</script>

<section class="grid gap-4">
	{#if !hasResult}
		<header>
			<h2 class="mt-2 mb-0 text-2xl text-black">Sala de espera de resultados</h2>
			<p class="mt-2 max-w-3xl text-zinc-700">
				Aún no se publican los resultados de este quiz. Te avisaremos cuando
				estén disponibles.
			</p>
		</header>

		<section class="panel-surface p-6 sm:p-8">
			<div class="flex flex-col items-center gap-4 text-center">
				<Loader2
					size={32}
					class="animate-spin text-zinc-600"
					aria-hidden="true"
				/>
				<p class="m-0 text-lg text-zinc-800">
					Esperando publicación de resultados
				</p>
				<p class="m-0 text-sm text-zinc-600">
					Próxima consulta en {nextCheckIn}s
				</p>

				{#if lastCheckedAt}
					<p class="m-0 text-xs text-zinc-500">
						Última consulta: {lastCheckedAt.toLocaleTimeString()}
					</p>
				{/if}

				<div class="flex gap-2">
					<button
						class="btn-secondary flex items-center gap-1.5"
						type="button"
						onclick={checkNow}
						disabled={isChecking}
					>
						<RefreshCw size={16} aria-hidden="true" />
						Consultar ahora
					</button>
					<button
						class="btn-tertiary flex items-center gap-1.5"
						type="button"
						onclick={goBack}
					>
						<ArrowLeft size={16} aria-hidden="true" />
						Volver
					</button>
				</div>
			</div>
		</section>
	{:else}
		<header>
			<h2 class="mt-2 mb-0 text-2xl text-black">Resultados disponibles</h2>
			<p class="mt-2 max-w-3xl text-zinc-700">
				Los resultados ya están publicados. Revisa tu desempeño a
				continuación.
			</p>
		</header>

		{#if result}
			<AttemptResultReview {result} />
		{/if}

		<div class="flex justify-end">
			<button
				class="btn-secondary flex items-center gap-1.5"
				type="button"
				onclick={goBack}
			>
				<ArrowLeft size={16} aria-hidden="true" />
				Volver
			</button>
		</div>
	{/if}
</section>
