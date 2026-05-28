export class RoleValue {
	private constructor(private readonly value: string) {}

	static from(value?: string): RoleValue | null {
		if (typeof value !== "string" || value.trim() === "") return null
		return new RoleValue(value.trim())
	}

	static format(value?: string): string {
		const grade = RoleValue.from(value)
		if (!grade) return "--"
		return grade.toDisplay()
	}

	toDisplay(): string {
		const labels: Record<string, string> = {
			student: "Estudiante",
			assistant: "Ayudante",
			func: "Profesor",
			admin: "Administrador",
		}

		return labels[this.value as keyof typeof labels] ?? "--"
	}
}
