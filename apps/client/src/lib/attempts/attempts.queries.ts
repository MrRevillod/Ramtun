import { createQuery } from "@tanstack/svelte-query"
import { attemptsService } from "$lib/attempts/attempts.service"

export const useActiveAttempt = () =>
	createQuery(() => ({
		queryKey: ["attempts", "active"] as const,
		queryFn: () => attemptsService.getActive(),
		retry: false,
		refetchOnMount: false,
		refetchOnWindowFocus: false,
	}))

export const useAttempt = (getAttemptId: () => string) =>
	createQuery(() => ({
		queryKey: ["attempts", "detail", getAttemptId()] as const,
		queryFn: () => attemptsService.getById(getAttemptId()),
		retry: false,
		enabled: !!getAttemptId(),
	}))
