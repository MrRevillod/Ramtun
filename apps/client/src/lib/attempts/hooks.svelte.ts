import type { Getter } from "runed"
import type { AttemptSession } from "./attempts.dtos"
import type { JoinQuizPreview } from "$lib/quizzes/quizzes.dtos"

import { goto } from "$app/navigation"
import { toast } from "svelte-sonner"
import { quizzesService } from "$lib/quizzes/quizzes.service"
import { attemptsService } from "./attempts.service"
import { getErrorMessage } from "$lib/shared/errors"
import { createMutation, createQuery } from "@tanstack/svelte-query"

export const useJoinAttemptPreviewQuery = (joinCode: Getter<string>) => {
	const query = createQuery(() => ({
		queryKey: ["join-preview", joinCode()],
		queryFn: () => quizzesService.joinByCodeOrThrow(joinCode()),
		enabled: joinCode().length > 0,
	}))

	const data = $derived(query.data)
	const isLoading = $derived(query.isLoading)
	const isError = $derived(query.isError)
	const error = $derived(query.error)

	return {
		data,
		isLoading,
		isError,
		error,
	}
}

export interface InitAttemptMutationOpts {
	joinCode: Getter<string>
	previewData: Getter<JoinQuizPreview | undefined>
}

export const useInitAttemptMutation = (input: InitAttemptMutationOpts) => {
	const mutation = createMutation(() => ({
		mutationFn: (quizId: string) => attemptsService.initializeOrThrow(quizId),
		onSuccess: async attempt => {
			if (!input.previewData()) {
				toast.error("No se pudo obtener la información del intento.")
				return
			}

			const session: AttemptSession = {
				joinCode: input.joinCode(),
				preview: input.previewData()!,
				attempt,
				answers: {},
				index: 0,
			}

			localStorage.setItem("last-attempt-session", JSON.stringify(session))

			await goto(`/attempts/${attempt.attemptId}`)
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const mutate = $derived(mutation.mutate)
	const mutateAsync = $derived(mutation.mutateAsync)
	const data = $derived(mutation.data)
	const error = $derived(mutation.error)
	const isPending = $derived(mutation.isPending)
	const isError = $derived(mutation.isError)
	const isSuccess = $derived(mutation.isSuccess)

	return {
		mutate,
		mutateAsync,
		data,
		error,
		isPending,
		isError,
		isSuccess,
	}
}
