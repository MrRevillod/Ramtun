export class GradeValue {
	private constructor(private readonly value: number) {}

	static from(value?: number | null): GradeValue | null {
		if (typeof value !== "number" || Number.isNaN(value)) return null
		return new GradeValue(value)
	}

	static format(value?: number | null): string {
		const grade = GradeValue.from(value)
		if (!grade) return "--"
		return grade.toDisplay()
	}

	toDisplay(): string {
		return new Intl.NumberFormat("es-CL", {
			minimumFractionDigits: 1,
			maximumFractionDigits: 1,
		}).format(this.value)
	}
}
