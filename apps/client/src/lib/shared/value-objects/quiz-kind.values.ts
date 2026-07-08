export class QuizKindValue {
	private constructor(private readonly value: string) {}

	static from(value?: string): QuizKindValue {
		if (typeof value !== "string" || value.trim() === "") {
			throw new Error("Invalid value for QuizKindValue")
		}

		return new QuizKindValue(value.trim())
	}

	static format(value?: string): string {
		const grade = QuizKindValue.from(value)
		if (!grade) return "--"
		return grade.toDisplay()
	}

	toDisplay(): string {
		const labels = {
			traditional: "Tradicional",
			certainty: "Certeza",
		}

		return labels[this.value as keyof typeof labels] ?? "--"
	}

	public isTraditional(): boolean {
		return this.value === "traditional"
	}

	public isCertainty(): boolean {
		return this.value === "certainty"
	}
}
