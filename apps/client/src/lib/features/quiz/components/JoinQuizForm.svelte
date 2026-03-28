<script lang="ts">
	import { createMutation } from '@tanstack/svelte-query'
	import { Field, Form, createForm } from '@formisch/svelte'
	import { KeyRound } from 'lucide-svelte'
	import { toast } from 'svelte-sonner'
	import { quizService } from '$lib/features/quiz/quiz.service'
	import { quizUiStore } from '$lib/features/quiz/quiz.store.svelte'
	import { JoinQuizSchema, type JoinQuizInput } from '$lib/features/quiz/schema'
	import { toUserMessage } from '$lib/shared/errors'

	const joinForm = createForm({ schema: JoinQuizSchema })

	const joinMutation = createMutation(() => ({
		mutationFn: (payload: { code: string }) => quizService.joinQuizPreview(payload)
	}))

	const handleJoinSubmit = async (output: JoinQuizInput) => {
		const { value, error } = await joinMutation.mutateAsync({ code: output.code })

		if (error) {
			toast.error(toUserMessage(error))
			return
		}

		quizUiStore.showJoinPreview(value)
		toast.success('Codigo valido. Revisa las instrucciones antes de comenzar.')
	}
</script>

<section class="panel-surface p-6 sm:p-7">
	<h3 class="m-0 flex items-center gap-2 text-xl text-black">
		<KeyRound size={18} class="text-black" />
		Unirse a un quiz existente
	</h3>
	<p class="mt-2 max-w-2xl text-base leading-relaxed text-zinc-700">
		Ingresa el codigo de union compartido por tu docente.
	</p>

	<Form class="mt-5 grid gap-4" of={joinForm} onsubmit={handleJoinSubmit}>
		<Field of={joinForm} path={['code']}>
			{#snippet children(field)}
				<label class="grid gap-1.5">
					<span class="text-base text-zinc-800">Codigo de union</span>
					<input
						{...field.props}
						class="input-base"
						type="text"
						value={field.input}
					/>
					{#if field.errors}
						<p class="m-0 text-base text-red-700">{field.errors[0]}</p>
					{/if}
				</label>
			{/snippet}
		</Field>

		<button
			class="btn-primary w-full sm:w-fit"
			type="submit"
			disabled={joinMutation.isPending}
		>
			{joinMutation.isPending ? 'Uniendose...' : 'Entrar'}
		</button>
	</Form>
</section>
