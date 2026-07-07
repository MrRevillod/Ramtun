<script lang="ts">
	import type { ManagedUser } from "$lib/users/users.dtos"
	import type { SubmitEventHandler } from "@formisch/svelte"

	import { Plus, X } from "lucide-svelte"
	import { fade, scale } from "svelte/transition"
	import { addCourseMemberSchema } from "$lib/courses/courses.dtos"
	import { createForm, Field, Form, reset } from "@formisch/svelte"

	interface AddMemberModalProps {
		open: boolean
		oncancel: () => void
		candidates: ManagedUser[]
		mutation: {
			isPending: boolean
			mutateAsync: (input: { userId: string }) => Promise<void>
		}
	}

	let { open, oncancel, candidates, mutation }: AddMemberModalProps = $props()

	const form = createForm({
		schema: addCourseMemberSchema,
		initialInput: { userId: "" },
	})

	const handleSubmit: SubmitEventHandler<typeof addCourseMemberSchema> = async output => {
		await mutation.mutateAsync(output)
		reset(form)
		oncancel()
	}
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4"
		role="dialog"
		aria-modal="true"
		tabindex="-1"
		transition:fade={{ duration: 180 }}
		onclick={oncancel}
		onkeydown={e => {
			if (e.key === "Escape") oncancel()
		}}
	>
		<section
			class="panel-elevated w-full max-w-xl p-5"
			role="presentation"
			tabindex="-1"
			transition:scale={{ duration: 190, start: 0.98 }}
			onclick={e => e.stopPropagation()}
		>
			<div class="mb-3 flex items-center justify-between gap-2">
				<h4 class="m-0 text-base text-black">Agregar miembro</h4>
				<button class="btn-tertiary p-1" type="button" onclick={oncancel}>
					<X size={18} aria-hidden="true" />
				</button>
			</div>

			<Form of={form} onsubmit={handleSubmit} class="form-stack">
				<Field of={form} path={["userId"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-sm text-zinc-800">Usuario</span>
							<select {...field.props} class="input-base" value={field.input ?? ""}>
								<option value="">Selecciona un usuario</option>
								{#each candidates as candidate (candidate.id)}
									<option value={candidate.id}>
										{candidate.username} · {candidate.name}
									</option>
								{/each}
							</select>
							{#if field.errors?.[0]}
								<span class="text-sm text-red-700">{field.errors[0]}</span>
							{/if}
						</label>
					{/snippet}
				</Field>

				<div class="flex justify-end gap-2">
					<button class="btn-tertiary" type="button" onclick={oncancel}>Cancelar</button>
					<button class="btn-primary flex items-center gap-1.5" type="submit">
						<Plus size={16} aria-hidden="true" />
						{mutation.isPending ? "Agregando..." : "Agregar miembro"}
					</button>
				</div>
			</Form>
		</section>
	</div>
{/if}
