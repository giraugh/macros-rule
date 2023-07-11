<script lang="ts">
	import Decree from '../lib/Decree.svelte'
	import Slide from '../lib/Slide.svelte'
	import Notes from '../lib/Notes.svelte'
	import Code from '../lib/Code.svelte'
	import Info from '../lib/Info.svelte'
	import { MoveDown } from 'lucide-svelte'
</script>

<Slide>
	<Decree>More macros! Do one with repetition!</Decree>
	<Notes>
		<p>Okay, okay. Yes Sire. Right on it!</p>
		<p>
			So far, we've seen how we can create a basic macro which helps to reduce repetition in much
			the same way as a function does. It's a *bit* more flexible than a function because we use
			tokens rather than value types but, there's still more we can do with macros.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`vec![1]\nvec![1, 2]\nvec![1, 2, 3]`} />
	<Info>Can be invoked with any argument count</Info>
	<Notes>
		Functions in rust always have a set number of parameters and they must be called with that
		number of parameters. This is in contrast to languages like C, which let you define variadic
		functions which can accept a varying number of arguments. However, rust <strong>macros</strong>
		can be variadic which helps to bridge this gap. This is why you can call macros like `vec!` with
		any number of elements or call <code>println!</code> with many format arguments.
	</Notes>
</Slide>

<Slide>
	<Code short collapse code={`$($x:expr),*`} />
	<Info>Matches any amount of expressions seperated by commas</Info>
	<Notes>
		<p>
			macro_rules has a feature called *repetition* that allows us to easily create macros which
			accept varying amount of arguments. Repetition lets a rule in our macro collect up multiple
			tokens as a single metavariable and then expand them back out in the transcriber.
		</p>
		<p>The syntax for this can get a little confronting, so hang in there!</p>
		<p>This pattern matches any amount of expressions seperated by commas. Lets break it down!</p>
	</Notes>
</Slide>

<Slide>
	<Code short collapse code={`$()`} />
	<Notes>
		<p>We start with a dollar sign followed by a set of parentheses. This denotes a repetition</p>
	</Notes>
</Slide>

<Slide>
	<Code short collapse code={`$($x:expr)`} />
	<Notes>
		<p>Inside this parentheses we put the rust fragments that we want to repeat.</p>
		<p>So for example, this is a repetition of expressions.</p>
	</Notes>
</Slide>

<Slide autoAnimate>
	<Code short collapse code={`$($x:expr),`} />
	<Notes>
		<p>
			Then, optionally, we can provide a seperator that should exist between each repeated fragment
		</p>
		<p>Here we use a comma</p>
	</Notes>
</Slide>

<Slide autoAnimate>
	<Code short collapse code={`$($x:expr),*`} />
	<Code fragment short collapse code={`$($x:expr),+`} />
	<Code fragment short collapse code={`$($x:expr)?`} />
	<Notes>
		<p>
			And thats the pattern done! We use a similar syntax inside the transcriber but that might be
			easier to show in practice. Lets create a simple macro to get to grips with repetitions
		</p>
	</Notes>
</Slide>

<Slide autoAnimate>
	<!-- TODO: add labels -->
	<Info>0 or more</Info>
	<Code smallGap short collapse code={`$($x:expr),*`} />
	<Info>1 or more</Info>
	<Code smallGap short collapse code={`$($x:expr),+`} />
	<Info>0 or 1</Info>
	<Code smallGap short collapse code={`$($x:expr)?`} />
	<Notes>
		<p>
			Finally, we add a repetition character. This controls how many times the fragments can be
			repeated.
		</p>
		<p>
			In this case we use an asterisk which denotes that the fragment can be matched any amount of
			times. We could also have used a plus to match at least 1 fragment or a question mark to match
			exactly 0 or 1 repetitions. Note that the question mark can't have a seperator.
		</p>
		<p>
			And thats the pattern done! We use a similar syntax inside the transcriber but that might be
			easier to show in practice. Lets create a simple macro to get to grips with repetitions
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`let t = tuple_of!(Some; 1, 2, 3);`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code collapse code={`let t = (Some(1), Some(2), Some(3));`} />
	</div>
	<Notes>
		<p>Here's the plan. We want a macro that we can call like this. and then when expanded...</p>
		<p>
			We get this. We can use anything in place of `Some` here, as long as its a type that accepts a
			single argument sort of like Some.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`let t = tuple_of!(1, 2, 3);`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code collapse code={`let t = (1, 2, 3);`} />
	</div>
	<Notes>
		<p>
			To start us off with though, lets create a macro which simply creates a tuple from its
			arguments.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {



}`}
	/>
	<Notes>
		<p>As before, we invoke macro rules to define our macro. Now lets add our rule.</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {
    ($($x:expr),*) => {

		};
}`}
	/>
	<Notes>
		<p>
			This pattern is the same one we were looking at before. It matches invocations with 0 or more
			expressions.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {
    ($($x:expr),*) => {
        ( )
		};
}`}
	/>
	<Notes>
		<p>We are going to create a tuple so lets start with some parens for the tuple.</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {
    ($($x:expr),*) => {
        ( $($x),* )
		};
}`}
	/>
	<Notes>
		<p>
			Now we can use a very similiar repetition syntax in the transcriber to expand the expressions
			back out. This expands out our expressions with commas between each.
		</p>
		<p>Alright, our macro is done. Let's give it a go!</p>
		<p>But how can we add that mapping feature?</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {
    ($($x:expr),*) => {
        ( $($x, 0),* )
		};
}`}
	/>
	<Notes>
		<p>
			Well, before we do let me highlight this. We can add extra syntax into our repetition to cause
			it to get repeated too.
		</p>
		<p>So here I've added an extra 0 element between every element.</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`let t = tuple_of!(1, 2, 3);`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code collapse code={`let t = (1, 0, 2, 0, 3, 0);`} />
	</div>
	<Notes>
		<p>And if we expand it, you can see we get the 0s coming through as well.</p>
		<p>Now lets setup our proper mapping syntax</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {
    ($($x:expr),*) => {
        ( $($x),* )
		};

    ($mapper:path; $($x:expr),*) => {

		};
}`}
	/>
	<Info><code>this::is::a::Path</code></Info>
	<Notes
		><p>
			We can start by adding a new pattern that accepts a path. A path can be a type or a path to a
			type but it also includes enum variants which we will want here.
		</p></Notes
	>
</Slide>

<Slide>
	<Code
		code={`macro_rules! tuple_of {
    ($($x:expr),*) => {
        ( $($x),* )
		};
		
    ($mapper:path; $($x:expr),*) => {
        ( $($mapper($x)),* )
		};
}`}
	/>
	<Notes
		><p>
			And then we wrap our $x variable with the mapper, so that all of our repetitions get mapped.
		</p></Notes
	>
</Slide>

<Slide>
	<Code collapse code={`let t = tuple_of!(Some; 1, 2, 3);`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code collapse code={`let t = (Some(1), Some(2), Some(3));`} />
	</div>
	<Notes>
		<p>
			Now if we invoke our macro with that pattern and expand it, you can see that the parameters
			are all mapped.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`struct MyStruct(usize);`} />
	<Code collapse fragment code={`let t = tuple_of!(MyStruct; 1, 2, 3);`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code collapse code={`let t = (MyStruct(1), MyStruct(2), MyStruct(3));`} />
	</div>
	<Notes>
		<p>
			And of course, we can use this with any struct or enum variant. So for example, we could
			create our own unit struct like this.
		</p>
	</Notes>
</Slide>
