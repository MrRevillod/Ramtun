import type { PageLoad } from "./$types"
import type { AttemptSession } from "$lib/attempts/attempts.dtos"

export const load: PageLoad = async ({ params }) => {
	let quizTitle = ""
	try {
		const raw = localStorage.getItem("last-attempt-session")
		if (raw) {
			const parsed = JSON.parse(raw) as AttemptSession
			if (parsed.attempt.attemptId === params.attemptId) {
				quizTitle = parsed.preview.title
			}
		}
	} catch {
		// localStorage unavailable or invalid data
	}

	return {
		attemptId: params.attemptId,
		quizTitle,
	}
}
