<script lang="ts">
	import { page } from "$app/stores"
	import { resolve } from "$app/paths"
	import { ArrowLeft, FileText, Shield } from "lucide-svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"

	let { data, children } = $props()

	const isResults = $derived($page.url.pathname.endsWith("/results"))
	const isSupervision = $derived($page.url.pathname.endsWith("/supervision"))

	const warningsQuery = createQuery(() => ({
		queryKey: ["attempt-warnings", data.attemptId],
		queryFn: () => attemptsService.getAttemptWarnings(data.attemptId),
	}))

	const warningBadge = $derived(warningsQuery.data?.length ?? 0)
</script>

<section class="mt-4 flex flex-col gap-3">
	<div class="flex flex-row items-center justify-center gap-3">
		<div class="w-1/3">
			<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">
				Revisión y Resultados
			</p>
			<h3 class="mt-0.5 mb-0 text-[22px] tracking-tight text-black">
				{data.quizName}
			</h3>
		</div>
		<div class="flex w-2/3 items-center justify-end gap-2">
			<a
				class="action-tab justify-center"
				data-active={isResults}
				href={resolve(
					`/courses/${data.courseId}/quizzes/${data.quizId}/attempts/${data.attemptId}/results`
				)}
			>
				<FileText size={16} /> Resultados
			</a>
			<a
				class="action-tab justify-center"
				data-active={isSupervision}
				href={resolve(
					`/courses/${data.courseId}/quizzes/${data.quizId}/attempts/${data.attemptId}/supervision`
				)}
			>
				<div class="flex items-center gap-1.5">
					<Shield size={16} /> Supervisión
					{#if warningBadge > 0}
						<span
							class="inline-flex h-5 min-w-5 items-center justify-center rounded-full bg-red-600 px-1.5 text-xs font-bold text-white"
						>
							{warningBadge}
						</span>
					{/if}
				</div>
			</a>
			<a
				class="action-tab shrink-0 justify-center"
				href={resolve(`/courses/${data.courseId}/quizzes/${data.quizId}/attempts`)}
			>
				<ArrowLeft size={16} aria-hidden="true" />
				Volver
			</a>
		</div>
	</div>

	<div class="mb-6 h-px bg-zinc-400/70"></div>
</section>
{@render children()}
