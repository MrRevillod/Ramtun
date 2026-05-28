<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { Users, Layers } from "lucide-svelte"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"

	const coursesQuery = createQuery(() => ({
		queryKey: ["admin", "courses"],
		queryFn: async () => await coursesService.list(),
	}))
</script>

<section class="grid gap-5">
	<header>
		<h2 class="mt-2 mb-0 text-2xl text-black">Panel de administracion</h2>
		<p class="mt-2 max-w-3xl text-zinc-700">
			Vista general de todos los cursos del sistema.
		</p>
	</header>

	<section class="panel-elevated p-4 sm:p-5">
		<h3 class="mt-0 mb-3 flex items-center gap-2 text-lg font-semibold text-black">
			<Layers size={18} aria-hidden="true" />
			Cursos
		</h3>
		{#if coursesQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando cursos...</p>
		{:else if coursesQuery.error}
			<p class="m-0 text-red-700">{getErrorMessage(coursesQuery.error)}</p>
		{:else if !coursesQuery.data?.length}
			<p class="notice notice-warn m-0">No hay cursos registrados.</p>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Codigo</th>
							<th class="px-3 py-2 text-left font-medium">Ano</th>
							<th class="px-3 py-2 text-left font-medium">Miembros</th>
						</tr>
					</thead>
					<tbody>
						{#each coursesQuery.data as course (course.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() => goto(resolve(`/courses/${course.id}/quizzes`))}
							>
								<td class="px-3 py-2 text-zinc-800">{course.name}</td>
								<td class="px-3 py-2 font-medium text-zinc-900">{course.code}</td>
								<td class="px-3 py-2 text-zinc-700">{course.year}</td>
								<td class="px-3 py-2 text-zinc-700">
									<span class="inline-flex items-center gap-1">
										<Users size={14} aria-hidden="true" />
										{course.members.length}
									</span>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>
