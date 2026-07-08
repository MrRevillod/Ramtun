export class DateValue {
	private constructor(public readonly value: string) {}

	static from(value?: string | null): DateValue | null {
		if (typeof value !== "string") return null
		return new DateValue(value)
	}

	static format(
		value?: string | null,
		timeStyle: Intl.DateTimeFormatOptions["timeStyle"] = "short"
	): string {
		const date = DateValue.from(value)
		if (!date) return "--"
		return date.toDisplay(timeStyle)
	}

	static default(): DateValue {
		return new DateValue(new Date().toISOString())
	}

	toDisplay(timeStyle: Intl.DateTimeFormatOptions["timeStyle"] = "short"): string {
		const parsed = new Date(this.value)

		if (Number.isNaN(parsed.getTime())) {
			return "Fecha no disponible"
		}

		return new Intl.DateTimeFormat("es-CL", {
			dateStyle: "medium",
			timeStyle,
		}).format(parsed)
	}
}
