<script lang="ts">
	import {
		createMutation,
		createQuery,
		useQueryClient,
	} from "@tanstack/svelte-query"
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import { toast } from "svelte-sonner"
	import { Plus, Trash2 } from "lucide-svelte"
	import { banksService } from "$lib/banks/banks.service"
	import { coursesService } from "$lib/courses/courses.service"
	import { getErrorMessage } from "$lib/shared/errors"
	import { DateValue } from "$lib/shared/value-objects/date.value"

	import BankUploadModal from "$lib/banks/components/BankUploadModal.svelte"
	import ConfirmActionModal from "$lib/shared/components/ConfirmActionModal.svelte"

	let { data } = $props()

	const queryClient = useQueryClient()

	const courseQuery = createQuery(() => ({
		queryKey: ["course", data.courseId],
		queryFn: () => coursesService.getOrThrow(data.courseId),
	}))

	const banksQuery = createQuery(() => ({
		queryKey: ["banks", data.courseId],
		queryFn: () => banksService.listByCourseOrThrow(data.courseId),
	}))

	let showCreateModal = $state(false)
	let bankToDelete = $state<{ id: string; name: string } | null>(null)

	const uploadBankMutation = createMutation(() => ({
		mutationFn: (input) => banksService.createOrThrow(input),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const deleteBankMutation = createMutation(() => ({
		mutationFn: (bankId: string) => banksService.softDeleteOrThrow(bankId),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ["banks", data.courseId] })
			bankToDelete = null
		},
		onError: error => toast.error(getErrorMessage(error)),
	}))

	const confirmDeleteBank = () => {
		if (!bankToDelete) return
		deleteBankMutation.mutate(bankToDelete.id)
	}
</script>

<section class="grid gap-4">
	<header>
		<div class="flex flex-wrap items-start justify-between gap-3">
			<div>
				<h3 class="mt-2 mb-0 text-xl text-black">
					{courseQuery.data?.name ?? "Curso"} - Bancos de preguntas
				</h3>
				<p class="m-0 mt-2 text-zinc-700">
					Sube y gestiona bancos de preguntas en formato JSON.
				</p>
			</div>
			<button
				class="btn-primary flex items-center gap-1.5"
				type="button"
				onclick={() => (showCreateModal = true)}
			>
				<Plus size={16} aria-hidden="true" />
				Nuevo banco
			</button>
		</div>
	</header>

	<section class="panel-elevated p-4">
		{#if banksQuery.isLoading}
			<p class="m-0 text-zinc-600">Cargando bancos...</p>
		{:else if banksQuery.isError}
			<p class="m-0 text-red-700">Error al cargar los bancos.</p>
		{:else if !banksQuery.data?.length}
			<div class="panel-surface py-10 text-center">
				<p class="mb-2 text-zinc-600">No hay bancos de preguntas.</p>
				<p class="text-sm text-zinc-500">
					Sube un archivo JSON para crear el primer banco.
				</p>
			</div>
		{:else}
			<div class="overflow-x-auto">
				<table class="min-w-full border-collapse text-sm">
					<thead class="table-head">
						<tr>
							<th class="px-3 py-2 text-left font-medium">Nombre</th>
							<th class="px-3 py-2 text-left font-medium">Preguntas</th>
							<th class="px-3 py-2 text-left font-medium">Creado</th>
							<th class="px-3 py-2 text-left font-medium">Acciones</th>
						</tr>
					</thead>
					<tbody>
						{#each banksQuery.data as bank (bank.id)}
							<tr
								class="row-hover table-row cursor-pointer"
								onclick={() =>
									goto(resolve(`/courses/${data.courseId}/banks/${bank.id}`))}
							>
								<td class="px-3 py-2 text-zinc-900">{bank.name}</td>
								<td class="px-3 py-2 text-zinc-700">{bank.questions.length}</td>
								<td class="px-3 py-2 text-zinc-700"
									>{DateValue.format(bank.created_at ?? bank.createdAt ?? "")}</td
								>
								<td class="px-3 py-2">
									<button
										class="icon-btn icon-btn-danger"
										title="Eliminar"
										type="button"
										onclick={e => {
											e.stopPropagation()
											bankToDelete = { id: bank.id, name: bank.name }
										}}
										disabled={deleteBankMutation.isPending}
									>
										<Trash2 size={15} aria-hidden="true" />
									</button>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</section>
</section>

<BankUploadModal
	open={showCreateModal}
	courseId={data.courseId}
	onclose={() => (showCreateModal = false)}
	onsuccess={() => (showCreateModal = false)}
	mutation={uploadBankMutation}
/>

<ConfirmActionModal
	open={!!bankToDelete}
	title="Eliminar banco"
	message={bankToDelete ? `Se eliminara el banco ${bankToDelete.name}.` : ""}
	confirmLabel="Eliminar"
	isPending={deleteBankMutation.isPending}
	onCancel={() => (bankToDelete = null)}
	onConfirm={confirmDeleteBank}
/>
