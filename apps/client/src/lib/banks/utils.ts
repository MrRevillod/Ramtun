import type { QuestionBank } from "$lib/banks/banks.dtos"

type QuestionExport = {
	prompt: string
	options: string[]
	answerIndex: number
	images: string[]
}

const sanitizeFileName = (name: string) =>
	name
		.trim()
		.replace(/[\\/:*?"<>|\s]+/g, "_")
		.replace(/^_+|_+$/g, "") || "banco"

export const exportBankJson = (bank: QuestionBank): void => {
	const questions: QuestionExport[] = bank.questions.map((question) => ({
		prompt: question.prompt,
		options: question.options,
		answerIndex: question.answer_index ?? question.answerIndex ?? 0,
		images: question.images,
	}))

	const blob = new Blob([JSON.stringify(questions, null, 2)], {
		type: "application/json",
	})

	const url = URL.createObjectURL(blob)
	const link = document.createElement("a")
	link.href = url
	link.download = `${sanitizeFileName(bank.name)}.json`
	link.click()
	URL.revokeObjectURL(url)
}
