<script lang="ts">
	import type { CertaintyTable } from "$lib/quizzes/quizzes.dtos"

	interface CertaintyTableViewProps {
		table: CertaintyTable
	}

	let { table }: CertaintyTableViewProps = $props()

	const levels = [
		{ label: "Baja", key: "low" as const },
		{ label: "Media", key: "medium" as const },
		{ label: "Alta", key: "high" as const },
	]
</script>

<table class="w-full border-collapse text-xs">
	<thead class="bg-zinc-100/90 text-zinc-700 dark:bg-zinc-900/80 dark:text-zinc-100">
		<tr>
			<th class="border border-zinc-300 px-2 py-1.5 text-left dark:border-zinc-700">Nivel</th>
			<th class="border border-zinc-300 px-2 py-1.5 text-left dark:border-zinc-700">
				Correcta
			</th>
			<th class="border border-zinc-300 px-2 py-1.5 text-left dark:border-zinc-700">
				Incorrecta
			</th>
		</tr>
	</thead>
	<tbody>
		{#each levels as level (level.key)}
			<tr>
				<td class="border border-zinc-300 bg-white px-2 py-1.5 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100">
					{level.label}
				</td>
				<td class="border border-zinc-300 bg-white px-2 py-1.5 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100">
					{table[level.key].correct}
				</td>
				<td class="border border-zinc-300 bg-white px-2 py-1.5 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100">
					{table[level.key].incorrect}
				</td>
			</tr>
		{/each}
	</tbody>
</table>
