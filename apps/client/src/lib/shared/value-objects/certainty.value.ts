export class CertaintyValue {
	private constructor(private readonly value: string) {}

	static from(value?: string): CertaintyValue | null {
		if (typeof value !== "string" || value.trim() === "") return null
		return new CertaintyValue(value.trim())
	}

	static format(value?: string): string {
		const grade = CertaintyValue.from(value)
		if (!grade) return "--"
		return grade.toDisplay()
	}

	toDisplay(): string {
		const labels = {
			low: "Baja",
			medium: "Media",
			high: "Alta",
		}

		return labels[this.value as keyof typeof labels] ?? "--"
	}
}
