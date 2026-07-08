import { attemptsService } from "$lib/attempts/attempts.service"
import { useQuery } from "$lib/shared/http/tanstack"

export const useAttempt = (getAttemptId: () => string) =>
	useQuery(() => ({
		queryKey: ["attempts", "detail", getAttemptId()] as const,
		queryFn: () => attemptsService.getById(getAttemptId()),
		retry: false,
		enabled: !!getAttemptId(),
	}))
