<script lang="ts">
	export let style: string = ''
</script>

<p {style}>
	<slot />
</p>

<style>
	p {
		color: var(--subtle);
	}
</style>
