import { PersistedState } from 'runed'
import type { AttemptAnswer, AttemptSnapshot, JoinQuizPreview } from '$lib/features/quiz/types'

class QuizUiStore {
	#activeAttempt = new PersistedState<AttemptSnapshot | null>('quiz-active-attempt', null, {
		storage: 'local',
		syncTabs: false
	})

	activePanel = $state<'join' | 'create' | 'mine' | 'assistants' | null>('join')
	isFormatModalOpen = $state(false)
	isJoinCodeModalOpen = $state(false)
	createdQuizJoinCode = $state<string | null>(null)
	joinPreview = $state<JoinQuizPreview | null>(null)
	currentQuestionIndex = $state(0)

	get activeAttempt() {
		return this.#activeAttempt.current
	}

	get joinedQuiz() {
		return this.activeAttempt?.quiz ?? null
	}

	get attemptId() {
		return this.activeAttempt?.attemptId ?? null
	}

	setPanel = (panel: 'join' | 'create' | 'mine' | 'assistants') => {
		this.activePanel = panel
	}

	openFormatModal = () => {
		this.isFormatModalOpen = true
	}

	closeFormatModal = () => {
		this.isFormatModalOpen = false
	}

	openJoinCodeModal = (joinCode: string) => {
		this.createdQuizJoinCode = joinCode
		this.isJoinCodeModalOpen = true
	}

	closeJoinCodeModal = () => {
		this.isJoinCodeModalOpen = false
		this.createdQuizJoinCode = null
	}

	showJoinPreview = (preview: JoinQuizPreview) => {
		this.joinPreview = preview
		this.currentQuestionIndex = 0
		this.activePanel = 'join'
	}

	startQuizAttempt = (attempt: AttemptSnapshot) => {
		this.#activeAttempt.current = attempt
		this.joinPreview = null
		this.currentQuestionIndex = 0
		this.activePanel = 'join'
	}

	leaveQuizAttempt = () => {
		this.#activeAttempt.current = null
		this.joinPreview = null
		this.currentQuestionIndex = 0
		this.activePanel = 'join'
	}

	clearJoinPreview = () => {
		this.joinPreview = null
	}

	clearAllStores = () => {
		this.#activeAttempt.current = null
		this.joinPreview = null
		this.createdQuizJoinCode = null
		this.isJoinCodeModalOpen = false
		this.currentQuestionIndex = 0
		this.activePanel = 'join'
	}

	upsertAnswer = (answer: AttemptAnswer) => {
		if (!this.activeAttempt) {
			return
		}

		const answers = [...this.activeAttempt.answers]
		const index = answers.findIndex((item) => item.questionId === answer.questionId)

		if (index >= 0) {
			answers[index] = answer
		} else {
			answers.push(answer)
		}

		this.#activeAttempt.current = {
			...this.activeAttempt,
			answers
		}
	}

	goToQuestion = (index: number) => {
		if (!this.joinedQuiz) {
			return
		}

		const maxIndex = this.joinedQuiz.questions.length - 1
		this.currentQuestionIndex = Math.min(Math.max(index, 0), maxIndex)
	}
}

export const quizUiStore = new QuizUiStore()
