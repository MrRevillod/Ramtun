import type { QuestionInput } from "$lib/banks/banks.dtos"

export type ParsedBank = {
	name: string
	questions: QuestionInput[]
}

type QuestionSection = {
	heading: string
	lines: string[]
}

const FENCE_RE = /^\s*```/
const HEADING_RE = /^#+\s+(.*)$/
const OPTION_RE = /^\s*-\s+\[([ xX])\]\s+(.+)$/

const fileNameWithoutExtension = (fileName: string): string => {
	const name = fileName.replace(/\.[^.]+$/, "")
	return name || "banco"
}

const questionLabel = (index: number) => `Pregunta ${index + 1}`

export const parseBankMarkdown = (text: string, fallbackName: string): ParsedBank => {
	const lines = text.split(/\r?\n/)

	let name = fallbackName
	const sections: QuestionSection[] = []
	let current: QuestionSection | null = null
	let inFence = false
	let sawHeading = false

	for (const line of lines) {
		if (FENCE_RE.test(line)) {
			inFence = !inFence
		}

		const headingMatch = !inFence ? HEADING_RE.exec(line) : null

		if (headingMatch) {
			const level = (line.match(/^#+/) as RegExpMatchArray)[0].length

			if (!sawHeading && level === 1) {
				name = headingMatch[1].trim()
				sawHeading = true
				current = null
				continue
			}

			sawHeading = true
			current = { heading: headingMatch[1].trim(), lines: [] }
			sections.push(current)
			continue
		}

		if (current) {
			current.lines.push(line)
		}
	}

	if (sections.length === 0) {
		throw new Error(
			"El archivo no contiene preguntas. Separa cada pregunta con un título (###)."
		)
	}

	const questions: QuestionInput[] = []

	for (const [sectionIndex, section] of sections.entries()) {
		const options: { checked: boolean; text: string }[] = []
		const body: string[] = []
		let inBodyFence = false

		for (const line of section.lines) {
			if (FENCE_RE.test(line)) {
				inBodyFence = !inBodyFence
				body.push(line)
				continue
			}

			const optionMatch = inBodyFence ? null : OPTION_RE.exec(line)

			if (optionMatch) {
				options.push({
					checked: optionMatch[1].toLowerCase() === "x",
					text: optionMatch[2].trim(),
				})
			} else {
				body.push(line)
			}
		}

		let prompt = body.join("\n").trim()
		if (!prompt && section.heading) {
			prompt = section.heading
		}

		if (options.length < 2 || options.length > 5) {
			throw new Error(`${questionLabel(sectionIndex)}: debe tener entre 2 y 5 opciones.`)
		}

		const checkedCount = options.filter((option) => option.checked).length
		if (checkedCount !== 1) {
			throw new Error(
				`${questionLabel(sectionIndex)}: debe marcar exactamente una opción correcta con [X].`
			)
		}

		questions.push({
			prompt,
			options: options.map((option) => option.text),
			answerIndex: options.findIndex((option) => option.checked),
		})
	}

	return { name, questions }
}

export const bankToMarkdown = (name: string, questions: QuestionInput[]): string => {
	const blocks = questions.map((question, index) => {
		const options = question.options
			.map(
				(option, optionIndex) =>
					`- [${optionIndex === question.answerIndex ? "X" : " "}] ${option}`
			)
			.join("\n")

		return `### ${index + 1}\n${question.prompt}\n\n${options}`
	})

	return `# ${name}\n\n${blocks.join("\n\n")}\n`
}

export { fileNameWithoutExtension }
