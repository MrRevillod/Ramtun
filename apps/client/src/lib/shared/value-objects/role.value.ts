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

	static default(): RoleValue {
		return new RoleValue("student")
	}

	toDisplay(): string {
		const labels: Record<string, string> = {
			student: "Estudiante",
			assistant: "Ayudante",
			func: "Profesor",
			admin: "Administrador",
		}

		return labels[this.value] ?? "--"
	}

	public isStudent(): boolean {
		return this.value === "student"
	}

	public isFunc(): boolean {
		return this.value === "func"
	}

	public isAdmin(): boolean {
		return this.value === "admin"
	}

	public isAssistant(): boolean {
		return this.value === "assistant"
	}
}
