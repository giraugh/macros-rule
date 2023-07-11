<script lang="ts">
	export let trim = false
	export let noescape = true
	export let lineStartFrom = undefined
	export let lineNumbers: string | boolean | undefined = undefined

	export let smallGap = false
	export let language: string = 'rust'
	export let short = false
	export let small = false
	export let collapse = false
	export let fragment = false
	export let error = false
	export let code: string = ''
</script>

<pre class:fragment class:short class:collapse class:small class:error class:smallGap>
	 <code
		class={language}
		data-trim={trim}
		data-noescape={noescape}
		data-ln-start-from={lineStartFrom}
		data-line-numbers={lineNumbers}>
      <slot />
{code}
	 </code>
</pre>

<style>
	pre:not(.smallGap) {
		margin-block-end: 3em;
	}

	pre.smallGap {
		margin-block-end: 2.5em;
	}

	pre code {
		max-height: 650px;
	}

	pre.short,
	pre.collapse {
		width: max-content;
	}

	pre:not(.short):not(.small) {
		font-size: 1em;
	}

	pre.short code {
		font-size: 2.5em !important;
		padding: 50px;
	}

	pre.small {
		font-size: 0.9em;
	}
	/* 
	pre.small code {
		font-size: 0.8em !important;
	} */

	pre.error {
		outline-color: var(--error);
	}
</style>
