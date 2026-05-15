<script lang="ts">
	import { Users, Library, FileText, ArrowLeft } from "lucide-svelte"
	import { page } from "$app/state"
	import { resolve } from "$app/paths"

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

<section class="grid gap-3">
	<nav class="panel-muted grid gap-2 rounded-md p-2 sm:grid-cols-4">
		<a
			class="action-tab flex w-full items-center justify-center gap-1.5 py-2.5"
			data-active={isQuizzes}
			href={resolve(quizzesHref)}
		>
			<FileText size={16} aria-hidden="true" />
			Quizzes
		</a>
		<a
			class="action-tab flex w-full items-center justify-center gap-1.5 py-2.5"
			data-active={isBanks}
			href={resolve(banksHref)}
		>
			<Library size={16} aria-hidden="true" />
			Bancos de preguntas
		</a>
		<a
			class="action-tab flex w-full items-center justify-center gap-1.5 py-2.5"
			data-active={isMembers}
			href={resolve(membersHref)}
		>
			<Users size={16} aria-hidden="true" />
			Miembros del curso
		</a>
		<a
			class="action-tab flex w-full items-center justify-center gap-1.5 py-2.5"
			href={resolve("/courses")}
		>
			<ArrowLeft size={16} aria-hidden="true" />
			Volver al listado de cursos
		</a>
	</nav>

	<div class="p-1 sm:p-2">
		{#key page.url.pathname}
			{@render children()}
		{/key}
	</div>
</section>
