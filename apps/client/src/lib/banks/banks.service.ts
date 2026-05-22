import { request } from "$lib/shared/http/http"
import type { CreateQuestionBankInput, QuestionBank } from "$lib/banks/banks.dtos"

class BanksService {
	public listByCourse(courseId: string): Promise<QuestionBank[]> {
		return request<QuestionBank[]>({
			method: "GET",
			url: `/banks/course/${courseId}`,
		})
	}

	public getById(bankId: string): Promise<QuestionBank> {
		return request<QuestionBank>({
			method: "GET",
			url: `/banks/${bankId}`,
		})
	}

	public create(input: CreateQuestionBankInput): Promise<void> {
		return request<void>({
			method: "POST",
			url: "/banks",
			data: input,
		})
	}

	public softDelete(bankId: string): Promise<void> {
		return request<void>({
			method: "DELETE",
			url: `/banks/${bankId}`,
		})
	}
}

export const banksService = new BanksService()
