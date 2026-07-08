import type { CourseDTO, CourseMemberDTO } from "./dtos"

export class CourseMember {
	constructor(
		public userId: string,
		public username: string,
		public name: string,
		public role: "assistant" | "func"
	) {}

	public static fromDTO(dto: CourseMemberDTO): CourseMember {
		return new CourseMember(dto.userId, dto.username, dto.name, dto.role)
	}
}

export class Course {
	constructor(
		public id: string,
		public name: string,
		public code: string,
		public year: number,
		public members: CourseMember[] = []
	) {}

	public static fromDTO(dto: CourseDTO): Course {
		const members = dto.members.map(CourseMember.fromDTO)
		return new Course(dto.id, dto.name, dto.code, dto.year, members)
	}
}
