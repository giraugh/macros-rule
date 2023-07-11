<script lang="ts">
	import scrollImg from '../assets/images/Scroll.png'
</script>

<div class="scroll" style:background-image={`url(${scrollImg})`}>
	<h4>
		<slot />
	</h4>
</div>

<style>
	.scroll {
		object-fit: contain;
		background: url(/src/assets/images/Scroll.png);
		background-size: cover;
		background-position: center center;
		width: 100%;
		height: 100%;
		min-height: 90vh;
		margin-block-start: -30px;

		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.scroll > h4 {
		display: block;
		width: 25%;
		padding: 1em;
		padding-block-end: 2.5em;
		font-weight: bold;
		font-size: 0.9em;
	}
</style>
