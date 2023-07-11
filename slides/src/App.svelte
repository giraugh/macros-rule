<script lang="ts">
	import 'reveal.js/dist/reveal.css'

	import Reveal from 'reveal.js'
	import { onMount, tick } from 'svelte'
	import Intro from './sections/01_Intro.svelte'
	import MakeOurOwn from './sections/02_MakeOurOwn.svelte'
	import Repetition from './sections/03_Repetition.svelte'
	import Recursive from './sections/04_Recursive.svelte'
	import Outro from './sections/05_Outro.svelte'
	export let revealOptions: Reveal.Options

	onMount(async () => {
		await tick()
		const deck = new Reveal(revealOptions)
		deck.initialize()
	})
</script>

<svelte:head>
	<title>Macros Rule!</title>
</svelte:head>

<div class="reveal">
	<div class="slides">
		<Intro />
		<MakeOurOwn />
		<Repetition />
		<Recursive />
		<Outro />
	</div>
</div>
