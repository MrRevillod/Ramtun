<script lang="ts">
	interface QuizOptionProps {
		text: string
		isSelected: boolean
		onclick: () => void
	}

	let { text, isSelected, onclick }: QuizOptionProps = $props()
</script>

<button type="button" class="quiz-option" data-active={isSelected} {onclick}>
	{text}
</button>
