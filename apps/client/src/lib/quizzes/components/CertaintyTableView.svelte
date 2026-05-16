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
	<thead class="bg-zinc-100/90 text-zinc-700">
		<tr>
			<th class="border border-zinc-300 px-2 py-1.5 text-left">Nivel</th>
			<th class="border border-zinc-300 px-2 py-1.5 text-left">Correcta</th>
			<th class="border border-zinc-300 px-2 py-1.5 text-left">Incorrecta</th>
		</tr>
	</thead>
	<tbody>
		{#each levels as level (level.key)}
			<tr>
				<td class="border border-zinc-300 bg-white px-2 py-1.5">{level.label}</td>
				<td class="border border-zinc-300 bg-white px-2 py-1.5">
					{table[level.key].correct}
				</td>
				<td class="border border-zinc-300 bg-white px-2 py-1.5">
					{table[level.key].incorrect}
				</td>
			</tr>
		{/each}
	</tbody>
</table>