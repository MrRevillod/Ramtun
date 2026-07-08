<script lang="ts" generics="TSchema extends Schema, TFieldPath extends RequiredPath">
	import {
		getInput,
		setInput,
		type FormStore,
		type RequiredPath,
		type Schema,
		type ValidPath,
	} from "@formisch/svelte"
	import { untrack } from "svelte"
	import type * as v from "valibot"

	type Props = {
		form: FormStore<TSchema>
		path: ValidPath<v.InferInput<TSchema>, TFieldPath>
	}

	let { form, path }: Props = $props()

	const MONTH_LABELS: Record<number, string> = {
		1: "Enero",
		2: "Febrero",
		3: "Marzo",
		4: "Abril",
		5: "Mayo",
		6: "Junio",
		7: "Julio",
		8: "Agosto",
		9: "Septiembre",
		10: "Octubre",
		11: "Noviembre",
		12: "Diciembre",
	}

	const daysInMonth = (year: number, month: number): number => {
		return new Date(year, month, 0).getDate()
	}

	const currentYear = new Date().getFullYear()
	const yearOptions = Array.from({ length: 6 }, (_, i) => currentYear + i)

	// Lee el valor inicial del form sin suscribirse a cambios (untrack).
	// El sync $effect de más abajo maneja cambios externos (ej. reset).
	//@ts-expect-error this
	const initialIso = untrack(() => getInput(form, { path: path as never }) as string | undefined)
	const initialDate =
		initialIso && !Number.isNaN(new Date(initialIso).getTime()) ? new Date(initialIso) : null

	let day = $state<number | null>(initialDate ? initialDate.getDate() : null)
	let month = $state(initialDate ? initialDate.getMonth() + 1 : 1)
	let year = $state(initialDate ? initialDate.getFullYear() : currentYear + 1)
	let localError = $state<string | null>(null)

	const maxDay = $derived(daysInMonth(year, month))

	$effect(() => {
		if (day !== null && day > maxDay) {
			day = null
		}
	})

	$effect(() => {
		if (day === null) {
			localError = "Selecciona un día"
			return
		}
		const date = new Date(year, month - 1, day)
		if (Number.isNaN(date.getTime())) {
			localError = "Fecha inválida"
			return
		}
		if (date <= new Date()) {
			localError = "La fecha de inicio debe estar en el futuro"
			return
		}
		localError = null
		setInput(form, { path: path as never, input: date.toISOString() as never })
	})

	$effect(() => {
		//@ts-expect-error path as never
		const isoString = getInput(form, { path: path as never }) as string | undefined
		if (!isoString) return
		const d = new Date(isoString)
		if (Number.isNaN(d.getTime())) return
		const dDay = d.getDate()
		const dMonth = d.getMonth() + 1
		const dYear = d.getFullYear()
		if (dDay !== day || dMonth !== month || dYear !== year) {
			day = dDay
			month = dMonth
			year = dYear
		}
	})
</script>

<div class="grid gap-1.5">
	<div class="form-grid-3 grid-cols-3">
		<label class="grid gap-1.5">
			<span class="text-sm text-zinc-800">Día</span>
			<select bind:value={day} class="input-base">
				<option value={null} disabled>Selecciona</option>
				{#each Array.from({ length: maxDay }, (_, i) => i + 1) as d (d)}
					<option value={d}>{d}</option>
				{/each}
			</select>
		</label>

		<label class="grid gap-1.5">
			<span class="text-sm text-zinc-800">Mes</span>
			<select bind:value={month} class="input-base">
				{#each Object.entries(MONTH_LABELS) as [num, label] (num)}
					<option value={Number(num)}>{label}</option>
				{/each}
			</select>
		</label>

		<label class="grid gap-1.5">
			<span class="text-sm text-zinc-800">Año</span>
			<select bind:value={year} class="input-base">
				{#each yearOptions as y (y)}
					<option value={y}>{y}</option>
				{/each}
			</select>
		</label>
	</div>

	{#if localError}
		<span class="text-sm text-red-700">{localError}</span>
	{/if}
</div>
