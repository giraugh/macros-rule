<script lang="ts">
	import Decree from '../lib/Decree.svelte'
	import Slide from '../lib/Slide.svelte'
	import Notes from '../lib/Notes.svelte'
	import Code from '../lib/Code.svelte'
	import Info from '../lib/Info.svelte'
	import { MoveRight } from 'lucide-svelte'

	import moreReadingImg from '../assets/images/More_Reading.png'
	import giraffeImg from '../assets/images/Giraffe.png'
</script>

<Slide>
	<Decree>Okay! That's enough macros for today!</Decree>
</Slide>

<Slide>
	<img src={moreReadingImg} style="max-height: 300px" alt="Book about macros" />
	<h2>More Macros!</h2>
	<ol style="list-style-type: upper-roman">
		<li>
			<Info
				><a href="https://doc.rust-lang.org/rust-by-example/macros.html">Rust by Example</a></Info
			>
		</li>
		<li>
			<Info
				><a href="https://doc.rust-lang.org/reference/macros-by-example.html"
					>The Rust Reference: Macros</a
				></Info
			>
		</li>
		<li>
			<Info
				><a href="https://www.youtube.com/watch?v=q6paRBbLgNw">Crust of Rust: Declarative Macros</a
				></Info
			>
		</li>
		<li>
			<Info
				><a href="https://veykril.github.io/tlborm/introduction.html"
					>The Little Book of Rust Macros</a
				></Info
			>
		</li>
	</ol>
	<Notes>
		<p>Okay well in that case, let me just leave you with some cool resources.</p></Notes
	>
</Slide>

<Slide>
	<img src={giraffeImg} style="max-height: 400px" alt="Giraffe wearing sunglasses" />
	<h2>Thank you!</h2>
	<Info>More from me <MoveRight /> <a href="https://ewanb.me">ewanb.me</a></Info>
	<Info>Check out my webcomic <MoveRight /> <a href="https://sas.ewanb.me">sas.ewanb.me</a></Info>
</Slide>
