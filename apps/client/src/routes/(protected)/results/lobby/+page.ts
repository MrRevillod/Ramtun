import type { PageLoad } from "./$types"

export const load: PageLoad = ({ url }) => {
	return {
		joinCode: url.searchParams.get("joinCode") ?? undefined,
		attemptId: url.searchParams.get("attemptId") ?? undefined,
	}
}
