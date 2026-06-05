<script lang="ts">
	import { page } from "$app/state"
	import { Users, Library, FileText } from "lucide-svelte"

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

<section class="flex flex-col gap-4 py-2">
	<section class="flex flex-col gap-3">
		<section class="flex flex-row items-center justify-center">
			<div class="w-1/3">
				<p class="m-0 text-xs font-semibold tracking-widest text-zinc-500">
					{data.course.code} - {data.course.year}
				</p>
				<h2 class="mt-0.5 mb-0 text-[22px] tracking-tight text-black">
					{data.course.name}
				</h2>
			</div>

			<nav class="mt-2.5 flex w-2/3 justify-end gap-2 rounded-md">
				<a
					class="action-tab min-w-0 shrink justify-center"
					data-active={isQuizzes}
					href={quizzesHref}
				>
					<FileText size={16} aria-hidden="true" />
					<span class="truncate">Quizzes</span>
				</a>
				<a
					class="action-tab min-w-0 shrink justify-center"
					data-active={isBanks}
					href={banksHref}
				>
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
			</nav>
		</section>

		<div class="h-px bg-zinc-400/70"></div>
	</section>

	<div>
		{#key page.url.pathname}
			{@render children()}
		{/key}
	</div>
</section>
