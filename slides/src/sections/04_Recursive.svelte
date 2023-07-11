<script lang="ts">
	import Decree from '../lib/Decree.svelte'
	import Slide from '../lib/Slide.svelte'
	import Notes from '../lib/Notes.svelte'
	import Code from '../lib/Code.svelte'
	import Info from '../lib/Info.svelte'
	import { MoveDown } from 'lucide-svelte'
</script>

<Slide>
	<Decree>Hey! You didn't finish that first macro!</Decree>
	<Notes
		><p>
			Oh right yeah! lets quickly finish that one! We have all the background knowledge we need know
		</p></Notes
	>
</Slide>

<Slide>
	<Code
		small
		code={`macro_rules! make_list {
			($element:expr) => {
          ListNode { value: $element, next: None }
			};
}`}
	/>
	<Code collapse code={`let list = make_list![10];`} />
	<Notes>
		<p>
			This was where we left it last. It currently only has a pattern which matches a single
			element. Lets fix that!
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		small
		code={`macro_rules! make_list {
    ($element:expr) => {
        ListNode { value: $element, next: None }
    };

    ($first:expr, $($other:expr),+) => {






    };
}`}
	/>
	<Notes>
		<p>
			Just like before, we want another pattern that can match multiple elements. The difference
			this time is that we are going to call this macro recursively so we need to seperately match
			the first macro in the list.
		</p>
		<p>
			Note that we are using a plus symbol to ensure that our list of other elements has at least
			one element
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		small
		code={`macro_rules! make_list {
    ($element:expr) => {
        ListNode { value: $element, next: None }
    };

    ($first:expr, $($other:expr),+) => {
        ListNode {
            value: $first,
            next: Some(Box::new(
                make_list!($($other),+)
            ))
        }
    };
}`}
	/>
	<Notes>
		<p>
			Alright and here's our transcriber. We create a node and use the first expression, then we
			call the same make_list macro recursively and pass it the remaining elements. Then it will
			recurse until there's only a single element and use the top pattern.
		</p>
		<p>Lets give it a go!</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`let list = make_list![1, 2];`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code
			small
			code={`let list = ListNode {
    value: 1,
    next: Some(Box::new(ListNode {
      value: 2,
      next: None
    })),
};`}
		/>
	</div>
	<Notes
		><p>Great! it works!</p>
		<p>Now we can tidy up our tests using this macro</p></Notes
	>
</Slide>

<Slide>
	<Code
		small
		code={`#[test]
fn sum_list_single() {
		let list = make_list![3];
		assert_eq!(sum(list), 3);
}`}
	/>
	<Code
		small
		code={`#[test]
fn sum_list_multiple() {
		let list = make_list![3, 5];
		assert_eq!(sum(list), 8);
}`}
	/>
	<Info>Improvement = they fit on one slide now 😎</Info>
</Slide>
