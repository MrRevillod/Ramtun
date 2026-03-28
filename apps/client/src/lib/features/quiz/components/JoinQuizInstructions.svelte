<script lang="ts">
	import { createMutation } from '@tanstack/svelte-query'
	import { ClipboardCheck, Play, TimerReset } from 'lucide-svelte'
	import { toast } from 'svelte-sonner'
	import { quizService } from '$lib/features/quiz/quiz.service'
	import { quizUiStore } from '$lib/features/quiz/quiz.store.svelte'
	import { toUserMessage } from '$lib/shared/errors'

	const preview = $derived(quizUiStore.joinPreview)

	const startAttemptMutation = createMutation(() => ({
		mutationFn: (quizId: string) => quizService.startAttempt(quizId)
	}))

	const formatDate = (value: string) =>
		new Intl.DateTimeFormat('es-CL', {
			dateStyle: 'medium',
			timeStyle: 'short'
		}).format(new Date(value))

	const handleStart = async () => {
		if (!preview) {
			return
		}

		const { value, error } = await startAttemptMutation.mutateAsync(preview.id)

		if (error) {
			toast.error(toUserMessage(error))
			return
		}

		quizUiStore.startQuizAttempt(value)
		toast.success('Intento iniciado correctamente.')
	}
</script>

{#if preview}
	<section class="panel-surface flex h-full min-h-0 flex-col gap-5 p-6 sm:p-7">
		<div class="space-y-2">
			<p class="section-kicker m-0">Instrucciones</p>
			<h3 class="m-0 text-2xl text-black">{preview.title}</h3>
			<p class="max-w-3xl text-sm leading-relaxed text-zinc-700 sm:text-base">
				Revisa los detalles antes de comenzar. Una vez iniciado el intento, las preguntas se mostraran
				en orden aleatorio y el tiempo lo controla el servidor.
			</p>
		</div>

		<div class="grid gap-3 sm:grid-cols-3">
			<div class="panel-muted p-4">
				<p class="m-0 text-xs uppercase tracking-[0.16em] text-zinc-600">Tipo</p>
				<p class="mt-2 text-lg text-black">
					{preview.kind === 'Certainly' ? 'Certeza' : 'Tradicional'}
				</p>
			</div>
			<div class="panel-muted p-4">
				<p class="m-0 text-xs uppercase tracking-[0.16em] text-zinc-600">Preguntas</p>
				<p class="mt-2 text-lg text-black">{preview.questionCount}</p>
			</div>
			<div class="panel-muted p-4">
				<p class="m-0 text-xs uppercase tracking-[0.16em] text-zinc-600">Duracion</p>
				<p class="mt-2 text-lg text-black">{preview.attemptDurationMinutes} min</p>
			</div>
		</div>

		<div class="panel-muted space-y-3 p-5">
			<p class="m-0 flex items-center gap-2 text-base font-medium text-black">
				<ClipboardCheck size={16} />
				Antes de comenzar
			</p>
			<ul class="space-y-2 pl-5 text-sm leading-relaxed text-zinc-700">
				<li>Inicio programado: {formatDate(preview.startTime)}</li>
				<li>Solo tienes un intento total para este quiz.</li>
				<li>Tus respuestas se guardan de forma incremental mientras avanzas.</li>
				<li>Si recargas la pagina, el sistema intentara reanudar tu intento en curso.</li>
			</ul>
		</div>

		<div class="mt-auto flex flex-wrap items-center justify-between gap-3">
			<button class="btn-tertiary" type="button" onclick={() => quizUiStore.clearJoinPreview()}>
				<TimerReset size={14} class="mr-1 inline" />
				Volver
			</button>
			<button class="btn-primary" type="button" onclick={handleStart} disabled={startAttemptMutation.isPending}>
				<Play size={14} class="mr-1 inline" />
				{startAttemptMutation.isPending ? 'Iniciando...' : 'Comenzar intento'}
			</button>
		</div>
	</section>
{/if}
