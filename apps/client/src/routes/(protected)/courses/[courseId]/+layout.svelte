<script lang="ts">
	import { Users, Library, FileText } from "lucide-svelte"

	let { children, data } = $props()

	const base = $derived(`/courses/${data.courseId}`)
</script>

<section class="grid gap-3">
	<nav class="panel-muted flex flex-wrap gap-2 p-2.5">
		<a class="action-tab flex items-center gap-1.5" href={`${base}/members`}>
			<Users size={16} aria-hidden="true" />
			Members
		</a>
		<a class="action-tab flex items-center gap-1.5" href={`${base}/banks`}>
			<Library size={16} aria-hidden="true" />
			Bancos
		</a>
		<a class="action-tab flex items-center gap-1.5" href={`${base}/quizzes`}>
			<FileText size={16} aria-hidden="true" />
			Quizzes
		</a>
	</nav>

	<div class="p-1 sm:p-2">{@render children()}</div>
</section>
