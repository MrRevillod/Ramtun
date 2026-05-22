import type {
	AnswerState,
	AttemptQuestion,
	CertaintyLevel,
} from "$lib/attempts/attempts.dtos"
import type { QuizKind } from "$lib/quizzes/quizzes.dtos"

export const isAnswered = (answer?: AnswerState | null) =>
	answer?.answerIndex !== undefined

export const isCertaintyComplete = (answer?: AnswerState | null) =>
	!!answer && answer.certaintyLevel !== null

export const canAdvance = (kind: QuizKind, answer?: AnswerState | null) => {
	if (!isAnswered(answer)) return false
	if (kind === "certainty") return isCertaintyComplete(answer)
	return true
}

export const canSaveAnswer = (kind: QuizKind, answer?: AnswerState | null) =>
	canAdvance(kind, answer)

export const firstUnansweredIndex = (
	questions: AttemptQuestion[],
	answers: Record<string, AnswerState>
) => questions.findIndex(q => answers[q.id]?.answerIndex === undefined)

export const firstCertaintyGapIndex = (
	questions: AttemptQuestion[],
	answers: Record<string, AnswerState>
) =>
	questions.findIndex(q => {
		const answer = answers[q.id]
		return !!answer && answer.certaintyLevel === null
	})
