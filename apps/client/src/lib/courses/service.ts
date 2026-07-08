import type { AddCourseMemberDTO, CourseDTO, CourseMemberDTO, CreateCourseDTO } from "./dtos"

import { http } from "$lib/shared/http/client"
import { Course, CourseMember } from "./entity"

class CoursesService {
	public async list(): Promise<Course[]> {
		const courses = http.request<CourseDTO[]>({
			method: "GET",
			url: "/courses",
		})

		return courses.then((courses) => courses.map(Course.fromDTO))
	}

	public async create(data: CreateCourseDTO): Promise<Course> {
		const course = http.request<CourseDTO>({
			method: "POST",
			url: "/courses",
			data,
		})

		return course.then((course) => Course.fromDTO(course))
	}

	public async findOne(courseId: string): Promise<Course> {
		const course = http.request<CourseDTO>({
			method: "GET",
			url: `/courses/${courseId}`,
		})

		return course.then((course) => Course.fromDTO(course))
	}

	public async listMembers(courseId: string): Promise<CourseMember[]> {
		const members = http.request<CourseMemberDTO[]>({
			method: "GET",
			url: `/courses/${courseId}/members`,
		})

		return members.then((members) => members.map(CourseMember.fromDTO))
	}

	public async isMember(courseId: string, userId: string): Promise<boolean> {
		return http.request<boolean>({
			method: "GET",
			url: `/courses/${courseId}/members/${userId}`,
		})
	}

	public async addMember(courseId: string, input: AddCourseMemberDTO): Promise<void> {
		return http.request<void>({
			method: "POST",
			url: `/courses/${courseId}/members`,
			data: input,
		})
	}

	public async removeMember(courseId: string, userId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/courses/${courseId}/members/${userId}`,
		})
	}

	public async remove(courseId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/courses/${courseId}`,
		})
	}
}

export const coursesService = new CoursesService()
