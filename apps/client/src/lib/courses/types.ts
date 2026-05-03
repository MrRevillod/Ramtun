export type Course = {
	id: string
	name: string
	code: string
	year: number
}

export type CourseMemberRole = "assistant" | "func" | "admin" | "student"

export type CourseMember = {
	userId: string
	username: string
	name: string
	role: CourseMemberRole
}

export type CourseView = {
	id: string
	name: string
	code: string
	year: number
	members: CourseMember[]
}

export type CreateCourseInput = {
	name: string
	code: string
	year: number
}

export type AddCourseMemberInput = {
	userId: string
}
