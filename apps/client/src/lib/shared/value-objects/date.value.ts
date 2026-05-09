export class DateValue {
	private constructor(private readonly value?: string) {}

	static from(value?: string | null): DateValue | null {
		if (typeof value !== "string") return null
		return new DateValue(value)
	}

	static format(value?: string | null): string {
		const date = DateValue.from(value)
		if (!date) return "--"
		return date.toDisplay()
	}

	toDisplay(): string {
		const parsed = new Date(this.value!)
		if (Number.isNaN(parsed.getTime())) {
			console.warn("[banks] invalid createdAt", { value: this.value })
			return "Fecha no disponible"
		}

		return new Intl.DateTimeFormat("es-CL", {
			dateStyle: "medium",
			timeStyle: "short",
		}).format(parsed)
	}
}
