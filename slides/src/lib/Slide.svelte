<script lang="ts">
	export let autoAnimate: boolean = false
</script>

<section {...autoAnimate ? { 'data-auto-animate': true } : {}}>
	<slot />
</section>
