<script lang="ts">
	import { page } from "$app/state"
	import { resolve } from "$app/paths"
	import { ArrowLeft, FileText, Shield } from "lucide-svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { attemptsService } from "$lib/attempts/attempts.service"
	import PageHeader from "$lib/shared/components/PageHeader.svelte"

	let { data, children } = $props()

	const isResults = $derived(page.url.pathname.endsWith("/results"))
	const isSupervision = $derived(page.url.pathname.endsWith("/supervision"))

	const warningsQuery = createQuery(() => ({
		queryKey: ["attempt-warnings", data.attemptId],
		queryFn: () => attemptsService.getAttemptWarnings(data.attemptId),
	}))

	const warningBadge = $derived(warningsQuery.data?.length ?? 0)
</script>

<section class="mt-4 mb-8 flex flex-col gap-3">
	<PageHeader supra="Revisión y Resultados" title={data.quizName} tag="h3">
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
	</PageHeader>
</section>
{@render children()}
