import type {
	AddCourseMemberInput,
	CourseMember,
	CourseView,
	CreateCourseInput,
} from "$lib/courses/courses.dtos"

import { http } from "$lib/shared/http/request"

class CoursesService {
	public list(): Promise<CourseView[]> {
		return http.request<CourseView[]>({
			method: "GET",
			url: "/courses",
		})
	}

	public create(input: CreateCourseInput): Promise<CourseView> {
		return http.request<CourseView>({
			method: "POST",
			url: "/courses",
			data: input,
		})
	}

	public get(courseId: string): Promise<CourseView> {
		return http.request<CourseView>({
			method: "GET",
			url: `/courses/${courseId}`,
		})
	}

	public listMembers(courseId: string): Promise<CourseMember[]> {
		return http.request<CourseMember[]>({
			method: "GET",
			url: `/courses/${courseId}/members`,
		})
	}

	public addMember(courseId: string, input: AddCourseMemberInput): Promise<void> {
		return http.request<void>({
			method: "POST",
			url: `/courses/${courseId}/members`,
			data: input,
		})
	}

	public removeMember(courseId: string, userId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/courses/${courseId}/members/${userId}`,
		})
	}

	public remove(courseId: string): Promise<void> {
		return http.request<void>({
			method: "DELETE",
			url: `/courses/${courseId}`,
		})
	}
}

export const coursesService = new CoursesService()
