<script lang="ts">
	let { text = "" }: { text?: string } = $props()
</script>

<div class="question-rich-text leading-relaxed text-pretty">{text}</div>

<style>
	.question-rich-text {
		white-space: pre-wrap;
	}
</style>
