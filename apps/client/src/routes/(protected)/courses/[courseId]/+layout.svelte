<script lang="ts">
	import { page } from "$app/state"
	import { Users, Library, FileText } from "lucide-svelte"
	import PageHeader from "$lib/shared/components/PageHeader.svelte"

	let { children, data } = $props()

	const base = $derived(`/courses/${data.courseId}`)
	const quizzesHref = $derived(`${base}/quizzes`)
	const banksHref = $derived(`${base}/banks`)
	const membersHref = $derived(`${base}/members`)
	const isQuizzes = $derived(
		page.url.pathname === base || page.url.pathname.startsWith(quizzesHref)
	)
	const isBanks = $derived(page.url.pathname.startsWith(banksHref))
	const isMembers = $derived(page.url.pathname.startsWith(membersHref))
</script>

<section class="flex flex-col py-2">
	<section class="flex flex-col">
		<PageHeader supra={`${data.course.code} - ${data.course.year}`} title={data.course.name}>
			<a
				class="action-tab min-w-0 shrink justify-center"
				data-active={isQuizzes}
				href={quizzesHref}
			>
				<FileText size={16} aria-hidden="true" />
				<span class="truncate">Cuestionarios</span>
			</a>
			<a class="action-tab min-w-0 shrink justify-center" data-active={isBanks} href={banksHref}>
				<Library size={16} aria-hidden="true" />
				<span class="truncate">Bancos de preguntas</span>
			</a>
			<a
				class="action-tab min-w-0 shrink justify-center"
				data-active={isMembers}
				href={membersHref}
			>
				<Users size={16} aria-hidden="true" />
				<span class="truncate">Miembros del curso</span>
			</a>
		</PageHeader>
	</section>

	<div>
		{#key page.url.pathname}
			{@render children()}
		{/key}
	</div>
</section>
