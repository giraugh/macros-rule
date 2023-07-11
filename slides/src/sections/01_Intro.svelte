<script lang="ts">
	import Title from '../lib/Title.svelte'
	import Decree from '../lib/Decree.svelte'
	import Slide from '../lib/Slide.svelte'
	import Notes from '../lib/Notes.svelte'
	import Code from '../lib/Code.svelte'
	import Info from '../lib/Info.svelte'

	import carImg from '../assets/images/Cargo_Expand_Cropped.png'

	import { MoveDown } from 'lucide-svelte'
</script>

<Slide>
	<Title />
	<Notes>
		<p>Macros Rule! but who rules Macros? Why, "king macro" of course.</p>
		<p>And he's right about to give his first decree of the day!</p>
	</Notes>
</Slide>

<Slide>
	<Decree>
		We need macros!
		<br />
		<br />
		The people of Rust must make more!
	</Decree>
	<Notes>
		<p>Right, we can do that. But first, what is a macro?</p>
	</Notes>
</Slide>

<Slide>
	{#each ['vec', 'println', 'env'] as snippet}
		<Code short fragment>
			{snippet}!()
		</Code>
	{/each}
	<Notes>
		<p>
			If you've used Rust before, you've almost certainly used a macro. You can always tell them
			apart from functions because they announce! themselves! very! clearly!
		</p>
		<p>
			What makes macros different to functions is that when you go to compile your code, *just
			before* the rest of your code is compiled, they *EXPAND* into more rust code.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code short>let nums = vec![1, 2, 3];</Code>
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code>
			{`
        let nums = <[_]>::into_vec(
          #[rustc_box] ::alloc::boxed::Box::new([1, 2, 3])
        );
      `}
		</Code>
	</div>
	<Notes>
		Have a look at this code. We use the vec! macro to create a new vector with a few elements. When
		this code is compiled, it expands out to become something like this.
	</Notes>
</Slide>

<Slide>
	<Code collapse language="bash">{`$> rustc -Zunpretty=expanded file.rs`}</Code>
	<Info>Will output expanded source to stdout</Info>
	<Notes>
		We can actually manually trigger this expansion ourself using `rustc`, the rust compiler.
	</Notes>
</Slide>

<Slide>
	<Code language="bash" code={`$> cargo install cargo-expand\n$> cargo expand`} />
	<img src={carImg} alt="Car expanding" />
	<Notes>
		Or, you can install the cargo extension `cargo expand` which gives you a nicer interface and
		more options. This can be great when working on more complicated macros.
	</Notes>
</Slide>
