import { request } from "$lib/shared/http/http"
import {
	unwrapResultOrThrow,
	type AppResult,
	type AppResultAsync,
} from "$lib/shared/result"
import type {
	AddCourseMemberInput,
	Course,
	CourseMember,
	CourseView,
	CreateCourseInput,
} from "$lib/courses/types"

class CoursesService {
	public list(): AppResultAsync<Course[]> {
		return request<Course[]>({
			method: "GET",
			url: "/courses",
		})
	}

	public async listOrThrow(): Promise<Course[]> {
		return unwrapResultOrThrow(await this.list())
	}

	public create(input: CreateCourseInput): AppResultAsync<CourseView> {
		return request<CourseView>({
			method: "POST",
			url: "/courses",
			data: input,
		})
	}

	public async createOrThrow(input: CreateCourseInput): Promise<CourseView> {
		return unwrapResultOrThrow(await this.create(input))
	}

	public get(courseId: string): AppResultAsync<CourseView> {
		return request<CourseView>({
			method: "GET",
			url: `/courses/${courseId}`,
		})
	}

	public async getOrThrow(courseId: string): Promise<CourseView> {
		return unwrapResultOrThrow(await this.get(courseId))
	}

	public listMembers(courseId: string): AppResultAsync<CourseMember[]> {
		return request<CourseMember[]>({
			method: "GET",
			url: `/courses/${courseId}/members`,
		})
	}

	public async listMembersOrThrow(courseId: string): Promise<CourseMember[]> {
		return unwrapResultOrThrow(await this.listMembers(courseId))
	}

	public addMember(
		courseId: string,
		input: AddCourseMemberInput
	): AppResultAsync<void> {
		return request<void>({
			method: "POST",
			url: `/courses/${courseId}/members`,
			data: input,
		})
	}

	public async addMemberOrThrow(
		courseId: string,
		input: AddCourseMemberInput
	): Promise<void> {
		return unwrapResultOrThrow(await this.addMember(courseId, input))
	}

	public removeMember(courseId: string, userId: string): AppResultAsync<void> {
		return request<void>({
			method: "DELETE",
			url: `/courses/${courseId}/members/${userId}`,
		})
	}

	public async removeMemberOrThrow(courseId: string, userId: string): Promise<void> {
		return unwrapResultOrThrow(await this.removeMember(courseId, userId))
	}

	public remove(courseId: string): AppResultAsync<void> {
		return request<void>({
			method: "DELETE",
			url: `/courses/${courseId}`,
		})
	}

	public async removeOrThrow(courseId: string): Promise<void> {
		return unwrapResultOrThrow(await this.remove(courseId))
	}
}

export const coursesService = new CoursesService()
