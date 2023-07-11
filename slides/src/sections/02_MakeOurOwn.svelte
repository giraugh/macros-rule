<script lang="ts">
	import Decree from '../lib/Decree.svelte'
	import Slide from '../lib/Slide.svelte'
	import Notes from '../lib/Notes.svelte'
	import Code from '../lib/Code.svelte'
	import Info from '../lib/Info.svelte'
	import { MoveDown } from 'lucide-svelte'

	import dragonImg from '../assets/images/Macro_Dragon.png'
	import macroBookImg from '../assets/images/Macro_Book.png'
	import cogsImg from '../assets/images/Cogs.png'
</script>

<Slide>
	<Decree>Hurry up!</Decree>
	<Notes>
		<p>Oh right, yep. Lets crack on with our own macro!</p>
		<p>Has anyone here made their own macros before?</p>
	</Notes>
</Slide>

<Slide>
	<!-- <Info>Lets unleash the power of <code>macro_rules!</code></Info> -->
	<div class="two-cols">
		<div>
			<img src={cogsImg} alt="Metalic rusty cogs" />
			<h2>Proc Macros</h2>
			<Info>Rust program</Info>
		</div>
		<div>or</div>
		<div>
			<img src={macroBookImg} alt="The Macro Book" />
			<h2>Macro Rules</h2>
			<Info>Declarative ruleset</Info>
		</div>
	</div>
	<Notes>
		<p>Okay, there are two primary ways of making macros in Rust.</p>
		<p>
			One way is to create a "procedural macro", which is effectively writing a rust program that
			takes source code as input and produces source code as output.
		</p>
		<p>
			The other way is to use a built-in macro called `macro_rules!` to create macros using a set of
			substitution rules.
		</p>
	</Notes>
</Slide>

<Slide>
	<img src={dragonImg} alt="Dragon breathing fire" />
	<Notes>
		<p>
			We are going to focus on macro_rules today because, while it isn't as powerful as proc macros,
			you can still achieve a lot with it!
		</p>
		<p>Let's try it out!</p>
	</Notes>
</Slide>

<Slide>
	<Code
		small
		code={`
struct ListNode {
	  /// Value at this point in the list
    value: usize,

		/// Optional next node
    next: Option<Box<ListNode>>,
}`}
	/>
	<Notes>
		<p>
			So, last month in the kingdom was tax month so the king's been hard at work adding up his
			profits.
		</p>
		<p>He's pretty old school so he still uses a linked list to do this.</p>
		<p>A linked list that looks like this</p>
		<p>It stores a bunch of usize values and an optional next node in the list.</p>
	</Notes>
</Slide>

<Slide>
	<Code
		small
		code={`#[test]
fn sum_list_single() {
		let list = ListNode {
				value: 3,
				next: None,
		};
		assert_eq!(sum(list), 3);
}`}
	/>
	<Notes>
		<p>
			He's been programming a long time but he still makes mistakes so its on me to write his unit
			tests.
		</p>
		<p>This one here is testing out our new `sum` routine with a single element</p>
		<p>It's fairly verbose but manageable</p>
	</Notes>
</Slide>

<Slide>
	<Code
		small
		code={`#[test]
fn sum_list_multiple() {
		let list = ListNode {
				value: 3,
				next: Some(Box::new(ListNode {
						value: 5,
						next: None,
				})),
		};
		assert_eq!(sum(list), 8);
}`}
	/>
	<Notes>
		<p>
			This test on the other hand is testing a sum with two elements and its already pretty wordy.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		small
		code={`#[test]
fn sum_list_multiple() {
		let list = ListNode {
				value: 3,
				next: Some(Box::new(ListNode {
						value: 5,
						next: Some(Box::new(ListNode {
								value: 1,
								next: None,
						})),
				})),
		};
		assert_eq!(sum(list), 9);
}`}
	/>
	<Notes>
		<p>
			And then with three its barely fitting on the slide! You can imagine how this scales with even
			more elements.
		</p>
		<p>This is a perfect opportunity to create a macro!</p>
	</Notes>
</Slide>

<Slide>
	<Code small code={`let list = make_list![10];`} />
	<Notes>
		<p>It's going to look something a little like this</p>
		<p>
			I always like to start by thinking about how I'll use my macro as it helps us think about how
			to make it.
		</p>
		<p>
			The advantage of a macro here, rather than a function is that we can take any amount of
			arguments and we don't have to worry about storing the values before they are a linked list.
			No need for a vec or an array. Plus its going to be very easy to use.
		</p>
		<p>Anyway, lets keep it simple and have our macro just take one argument for now.</p>
	</Notes>
</Slide>

<Slide>
	<Info style="opacity: 0;">hi</Info>
	<Code
		small
		code={`macro_rules! make_list {



}`}
	/>
	<Notes>
		<p>lets get started!</p>
		<p>
			To start with, we call the `macro_rules!` macro (dont forget the exclamation mark) and provide
			our macros name (without an exclamation mark)
		</p>
	</Notes>
</Slide>

<Slide>
	<Info><code>$metavariable:fragment_specifier</code></Info>
	<Code
		small
		code={`macro_rules! make_list {
		($element:expr) => {

		};
}`}
	/>
	<Notes>
		<p>
			Now we can add the first rule to our macros rules. Macro rules works a bit like a rust `match`
			statement but instead of matching types or values, we match little fragments of rust syntax.
		</p>
		<p>
			The pattern for this rule is super simple, we want it to match when our macro is invoked with
			a single expression. We then give that expression the name "element". These name bindings to
			tokens are known as "metavariables". So in this case we have one metavariable, element, and it
			has a "fragment specifier" (the part of rust syntax it matches) of an "expression".
		</p>
		<p>
			Okay so, if this pattern matches then our macro invocation will expand to become whatever we
			put between these braces. Lets fill it in!
		</p>
	</Notes>
</Slide>

<Slide>
	<Info style="opacity: 0;">hi</Info>
	<Code
		small
		code={`macro_rules! make_list {
		($element:expr) => {
				ListNode { value: $element, next: None }
		};
}`}
	/>
	<Notes>
		<p>
			So here we have the code which our macro will expand to become when invoked if the pattern
			matches. This is sometimes known as the rules "transcriber".
		</p>
		<p>I do want to quickly point out a few things.</p>
		<p>
			One; is that we use our element metavariable in the transcriber, it will get substited out for
			whatever the macro was invoked with.
		</p>
		<p>
			Two; is that our transcriber isn't terminated with a semicolon. This is because our macro can
			be used anywhere, most likely in our case it will be used as part of an expression, so we
			don't want to end that expression early. It's easy to get tripped up by the braces but
			remember that they just denote the start and end of the transcriber and aren't a block. If we
			wanted a block with multiple statements we would need to add an additional set of braces.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse code={`let list = make_list![10];`} />
	<div class="fragment">
		<MoveDown size="75" color="var(--subtle)" style="margin-block-end: 1em;" />
		<Code collapse code={`let list = ListNode { value: 10, next: None };`} />
	</div>
	<Notes>
		<p>Et voila! We have our very simple macro done. Lets see what it expands to!</p>
		<p>
			Very nice! I just want to also quickly point out that the square brackets on the call to
			make_list are arbitrary. We can use parentheses or even braces. Also, that 10 could be any
			rust expression; a maths expression or even a function call - but it has to be an expression.
			for example, it couldn't be a type.
		</p>
	</Notes>
</Slide>

<!--
<Slide>
	<Code
		code={`#[test]
fn new_tax_is_higher() {
		assert!(new_tax(5) > old_tax(5));
		assert!(new_tax(10) > old_tax(10));
		// ...
}`}
	/>
	<Notes>
		<p>
			Here's some code I made for the king the other week. It's a set of tests for our new tax
			calculation routines. We want to test that the new function always returns a higher amount of
			tax than the old one.
		</p>
		<p>
			<em>(It's not very popular with the townspeople)</em>
		</p>
		<p>
			You can see that we are actually already using a macro. The `assert!` macro is part of rust
			and will throw an error if the expression we pass it is false.
		</p>
		<p>
			Each of our assertions is that the new tax is higher than the old tax, so we could create a
			macro to do this pattern for us.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse>assert_gt!(new_tax(5), old_tax(5))</Code>
	<Notes>
		<p>
			We are gonna call our macro `assert_gt` and we'll want to call it like this and have it throw
			a nicer error if the new tax isn't higher than the old.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! assert_gt {

}`}
	/>
	<Notes>
		To start with, we call the `macro_rules!` macro (dont forget the exclamation mark) and provide
		our macros name (without the exclamation mark)
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! assert_gt {
			($a:expr, $b:expr) => {};
}`}
	/>
	<Notes>
		<p>
			Now we can add the first rule to our macros rules. Macro rules works a bit like a rust `match`
			statement but it uses patterns of rust tokens (bits of rust syntax) instead of patterns of
			types.
		</p>
		<p>
			So here you can see we are matching against a call to assert_gt! that passes two expressions
			seperated by a comma. We then bind those expressions to two "metavariables" called a and b.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! assert_gt {
			($a:expr, $b:expr) => {
				assert!($a > $b)
			};
}`}
	/>
	<Notes>
		<p>
			Now we can add the body of this rule. When the pattern matches, the macro invocation will
			expand to become anything we put inbetween the braces, with the metavariables substituted out.
		</p>
		<p>The body of this macro just calls the assert macro with the operands.</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`macro_rules! assert_gt {
			($a:expr, $b:expr) => {
				assert!($a > $b, "Expected {} > {}", $a, $b)
			};
}`}
	/>
	<Notes>
		<p>And then just for fun, we can use our metavariables twice to throw a nicer error.</p>
		<p>Now we are ready to use our macro in our tests!</p>
	</Notes>
</Slide>

<Slide>
	<Code
		code={`#[test]
fn new_tax_is_higher() {
		assert_gt!(new_tax(5), old_tax(5));
		assert_gt!(new_tax(10), old_tax(10));
		// ...
}`}
	/>
	<Notes>
		<p>Nice! It's in place and no compiler errors :&rpar;</p>
		<p>
			This isn't a super great use case for a macro but its more designed to show you that they can
			drop in and replace code you already have. Plus, they are flexible with types as they simply
			expand to more rust code. We could use any types in here that can be compared with the greater
			than operator.
		</p>
	</Notes>
</Slide>

<Slide>
	<Code collapse>{`($a:expr, $b:expr) => { ... `}</Code>
	<Code collapse fragment>{`($a:expr; $b:expr) => { ... `}</Code>
	<Code collapse fragment>{`($a:expr => $b:expr) => { ... `}</Code>
	<Notes>
		Just as a quick aside, the choice of the comma between the two expressions in our pattern was
		somewhat arbitrary. We could have used a semicolon for example, or even an arrow.
	</Notes>
</Slide>

<Slide>
	<Code collapse>{`($a:expr # $b:expr) => { ... `}</Code>
	<Code
		fragment
		error
		small
		code={`error: \`$a:expr\` is followed by \`#\`, which is not allowed for \`expr\` fragments
		--\> tax-test-macro/src/lib.rs:17:18
		 |
	17 |         ($a:expr # $b:expr) => {
		 |                  ^ not allowed after \`expr\` fragments
		 |
		 = note: allowed there are: \`=>\`, \`,\` or \`;\``}
	/>
	<Notes>
		<p>
			However, we can't use any symbol, we can only create patterns that represent valid rust
			syntax.
		</p>
		<p>
			So for example we couldn't use a hash (#) symbol in-between the expressions. Fortunately
			though, when it errors, the compiler will tell you which options are valid.
		</p>
	</Notes>
</Slide> -->

<style>
	.two-cols {
		display: grid;
		grid-template-columns: 1fr auto 1fr;
		align-items: start;
		gap: 2em;
	}

	.two-cols :nth-child(2) {
		align-self: center;
	}
</style>
