import { request } from "$lib/shared/http/http"
import { unwrapResultOrThrow, type AppResultAsync } from "$lib/shared/result"
import type { CreateQuestionBankInput, QuestionBank } from "$lib/banks/types"

class BanksService {
	public listByCourse(courseId: string): AppResultAsync<QuestionBank[]> {
		return request<QuestionBank[]>({
			method: "GET",
			url: `/banks/course/${courseId}`,
		})
	}

	public async listByCourseOrThrow(courseId: string): Promise<QuestionBank[]> {
		return unwrapResultOrThrow(await this.listByCourse(courseId))
	}

	public create(input: CreateQuestionBankInput): AppResultAsync<void> {
		return request<void>({
			method: "POST",
			url: "/banks",
			data: input,
		})
	}

	public async createOrThrow(input: CreateQuestionBankInput): Promise<void> {
		return unwrapResultOrThrow(await this.create(input))
	}

	public softDelete(bankId: string): AppResultAsync<void> {
		return request<void>({
			method: "DELETE",
			url: `/banks/${bankId}`,
		})
	}

	public async softDeleteOrThrow(bankId: string): Promise<void> {
		return unwrapResultOrThrow(await this.softDelete(bankId))
	}
}

export const banksService = new BanksService()
