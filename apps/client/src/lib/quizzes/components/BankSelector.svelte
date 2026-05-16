<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { banksService } from "$lib/banks/banks.service"

	interface BankSelectorProps {
		courseId: string
		selectedBankIds: string[]
		onchange: (bankId: string, selected: boolean) => void
	}

	let { courseId, selectedBankIds, onchange }: BankSelectorProps = $props()

	const banksQuery = createQuery(() => ({
		queryKey: ["banks", courseId],
		queryFn: () => banksService.listByCourseOrThrow(courseId),
	}))
</script>

{#if banksQuery.isLoading}
	<p class="m-0 text-sm text-zinc-600">Cargando bancos...</p>
{:else if banksQuery.error}
	<p class="m-0 text-sm text-red-700">{banksQuery.error}</p>
{:else if !banksQuery.data?.length}
	<p class="m-0 text-sm text-zinc-600">No hay bancos disponibles.</p>
{:else}
	<div class="grid gap-2 sm:grid-cols-2">
		{#each banksQuery.data as bank (bank.id)}
			<label
				class="flex items-center gap-2 rounded-[4px] border border-zinc-300 bg-white px-3 py-2"
			>
				<input
					type="checkbox"
					checked={selectedBankIds.includes(bank.id)}
					onchange={event =>
						onchange(bank.id, (event.currentTarget as HTMLInputElement).checked)}
				/>
				<span class="text-sm text-zinc-800">{bank.name}</span>
			</label>
		{/each}
	</div>
{/if}
