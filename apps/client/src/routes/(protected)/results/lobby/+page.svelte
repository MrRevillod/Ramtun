<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { createQuery } from "@tanstack/svelte-query"
	import { onMount } from "svelte"
	import { Loader2, RefreshCw, ArrowLeft, BadgeCheck } from "lucide-svelte"
	import type { AttemptResult } from "$lib/attempts/types"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import AttemptResultReview from "$lib/attempts/components/AttemptResultReview.svelte"

	let { data } = $props()

	const joinCode = $derived(data.joinCode as string | undefined)

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
				const r = await attemptsService.getResultsByJoinCodeOrThrow(joinCode)
				return r
			} catch (e) {
				const maybe = e as { kind?: string; status?: number }
				if (maybe.kind === "http" && maybe.status === 409) {
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
			const r = await attemptsService.getResultsByJoinCodeOrThrow(joinCode!)
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

<section class="grid gap-5">
	{#if !hasResult}
		<header>
			<h2 class="mt-2 mb-0 text-2xl text-black">Sala de espera de resultados</h2>
			<p class="mt-2 max-w-3xl text-zinc-700">
				Aun no se publican los resultados de este quiz. Esta pantalla se actualiza
				automaticamente.
			</p>
		</header>

		<section class="panel-elevated p-6 sm:p-8">
			<div class="flex flex-col items-center gap-4 text-center">
				<Loader2 size={32} class="animate-spin text-zinc-600" aria-hidden="true" />
				<p class="m-0 text-lg text-zinc-800">Esperando publicación de resultados</p>
				<p class="notice notice-warn m-0 text-sm">
					Próxima consulta en {nextCheckIn}s
				</p>

				{#if lastCheckedAt}
					<p class="m-0 text-xs text-zinc-500">
						Última consulta: {lastCheckedAt.toLocaleTimeString()}
					</p>
				{/if}

				<div class="flex flex-wrap justify-center gap-2">
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

				<p class="mt-2 mb-0 max-w-xl text-xs text-zinc-500">
					Puedes cerrar esta pantalla y volver luego con el mismo codigo. Si el
					resultado ya esta publicado, lo veras al instante.
				</p>
			</div>
		</section>
	{:else}
		<header>
			<h2 class="mt-2 mb-0 text-2xl text-black">Resultados disponibles</h2>
			<p class="mt-2 max-w-3xl text-zinc-700">
				Los resultados ya estan publicados. Revisa tu desempeno a continuacion.
			</p>
		</header>

		<p class="notice notice-ok m-0 inline-flex w-fit items-center gap-1.5">
			<BadgeCheck size={16} aria-hidden="true" />
			Tu resultado esta disponible para revision.
		</p>

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
