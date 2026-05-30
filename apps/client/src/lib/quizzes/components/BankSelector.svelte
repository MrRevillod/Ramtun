<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { banksService } from "$lib/banks/banks.service"
	import { ChevronDown, Search } from "lucide-svelte"

	interface BankSelectorProps {
		courseId: string
		selectedBankIds: string[]
		onchange: (bankId: string, selected: boolean) => void
	}

	let { courseId, selectedBankIds, onchange }: BankSelectorProps = $props()
	let open = $state(false)
	let search = $state("")

	const banksQuery = createQuery(() => ({
		queryKey: ["banks", courseId],
		queryFn: () => banksService.listByCourse(courseId),
	}))

	const filteredBanks = $derived(
		(banksQuery.data ?? []).filter(b =>
			b.name.toLowerCase().includes(search.toLowerCase()),
		),
	)

	const selectedCount = $derived(selectedBankIds.length)

	const handleToggle = (bankId: string) => {
		onchange(bankId, !selectedBankIds.includes(bankId))
	}
</script>

{#if banksQuery.isLoading}
	<p class="m-0 text-sm text-zinc-600">Cargando bancos...</p>
{:else if banksQuery.error}
	<p class="m-0 text-sm text-red-700">{banksQuery.error}</p>
{:else if !banksQuery.data?.length}
	<p class="m-0 text-sm text-zinc-600">No hay bancos disponibles.</p>
{:else}
	<div class="relative">
		<button
			class="input-base flex w-full cursor-pointer items-center justify-between gap-2"
			type="button"
			onclick={() => (open = !open)}
		>
			<span class="text-sm" class:text-zinc-400={!selectedCount}>
				{selectedCount
					? `${selectedCount} banco${selectedCount !== 1 ? "s" : ""} seleccionado${selectedCount !== 1 ? "s" : ""}`
					: "Seleccionar bancos"}
			</span>
			<ChevronDown size={16} class="text-zinc-400 shrink-0" />
		</button>

		{#if open}
			<div
				class="absolute left-0 right-0 top-full z-50 mt-1 max-h-64 overflow-auto rounded-md border border-zinc-300 bg-white shadow-lg"
			>
				<div class="sticky top-0 border-b border-zinc-200 bg-white p-2">
					<div class="relative">
						<Search
							size={14}
							class="pointer-events-none absolute left-2 top-1/2 -translate-y-1/2 text-zinc-400"
						/>
						<input
							class="input-base w-full pl-7 text-sm"
							type="text"
							placeholder="Buscar banco..."
							bind:value={search}
						/>
					</div>
				</div>

				{#each filteredBanks as bank (bank.id)}
					<label
						class="flex cursor-pointer items-center gap-2 px-3 py-2 hover:bg-zinc-50"
					>
						<input
							type="checkbox"
							checked={selectedBankIds.includes(bank.id)}
							onchange={() => handleToggle(bank.id)}
						/>
						<span class="text-sm text-zinc-800">{bank.name}</span>
					</label>
				{/each}

				{#if !filteredBanks.length}
					<p class="p-3 text-sm text-zinc-500">Sin resultados</p>
				{/if}
			</div>

			<div
				class="fixed inset-0 z-40"
				role="presentation"
				onclick={() => (open = false)}
			></div>
		{/if}
	</div>
{/if}
