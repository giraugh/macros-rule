# Macros Rule

## Introduction

Macros Rule!
but who rules Macros? Why, "king macro" of course.

And he's right about to give his first decree of the day!

> **We need macros! Get the people of rust to make more!**

Right, we can do that.

But quickly, what is a macro?
If you've used Rust before, you've almost certainly used a macro.
You can always tell them apart from functions because they announce! themselves! very! clearly!

What makes macros different to functions is that when you go to compile your code, *just before* the rest
of your code is compiled, they *EXPAND* into more rust code.

> [[ Slide has an example of using `vec![]` ]]

Have a look at this code. We use the vec! macro to create a new vector with a few elements.
When this code is compiled, it expands out to become something like this.

> [[ Slide shows expanded code ]]

We can actually do this expansion ourself using `rustc`, the rust compiler.
If you pass it the argument `-Zunpretty=expanded` you can show the expanded version.

> [[ slide shows a drake style meme. he says no to car on road but yes to car expanding ]]

Or, you can install the cargo extension `cargo expand` which gives you a nicer interface and more options.
This can be great when working on more complicated macros.

## Making our own macros

> **Hurry up!**

Oh right, yep. Lets get to making our own macros. 

Has anyone here made their own macros before? (Can raise hand)

> [[ Slide has proc macros, locked behind big door, and macro rules looking serene ]]

Okay, there are two primary ways of making macros in Rust.

One way is to create a "procedural macro", which is effectively writing a rust program that
takes source code as input and produces source code as output.

The other way is to use a built-in macro called `macro_rules!` to create macros
macro using a set of substitution rules.

We are going to focus on macro_rules today because, while it isn't as powerful as proc macros,
you can still achieve a lot with it!

Let's try it out!

## Example Macro 1 - assert_gt

Here's some code I made for the king last week. It's a set of tests for our new
tax calculation routines. (It's not very popular with the townspeople)

> [[ Slide shows code for testing tax calculation code. There's a lot of repetition ]]

You can see that all of our assertions are comparing two expressions. In each case we want the first
expression to be less than the second. There's a lot of repetition here and there are many ways we could clean
that up, but for now, lets make a simple macro.

> [[ slide shows calling `assert_gt!` ]]

We are going to call our macro `assert_gt` and have it take two arguments. We want to call it like this.

> [[ bare macro rules invocation ]]

To start with, we call the `macro_rules!` macro (dont forget the exclamation mark) and provide
our macros name (without the exclamation mark)

> [[ adding first rule ]]

Now we can add the first rule to our macros rules. Macro rules works a bit like a rust `match` statement
but it uses patterns of rust tokens (bits of rust syntax) instead of patterns of types.

We want the pattern of our rule to take two expressions seperated by a comma. The code in the curlies after the
arrow is what our macro will expand to when this pattern matches.

> [[ adding body to first rule ]]

The body of this rule will simply call `assert!` with each argument and then print a nice error message
if the assertion fails.

Alrighty! Our macro is done and we can use it in our test.

> [[ test rewritten using macro ]]

Nice! We've reduced the repetition a bit and we'll get nice errors now too.

> [[ macro pattern rewritten with a semicolon instead of comma ]]

Just as a quick aside, the choice of the comma between the two expressions in our pattern was somewhat
arbitrary. We could have used a semicolon for example.

However, we can't use any symbol, we can only create patterns that represent valid rust syntax.

> [[ Using a # and getting an error ]]

So for example we couldn't use a hash (#) symbol in-between the expressions.
When it errors, the compiler will tell you which options are valid.

## Repetition in macro rules

Alright, one macro down!

> **More macros! Do one with variadics!**

Okay, okay. Yes sire. Right on it.

So far we've created a macro that reduces repetition but we can do more with our custom macros.
You might be aware of macros which enable you to have varying amounts of parameters.

> [ Slide showing how you can call vec with different amounts of arguments ]

For example, with vec!, we can create a vector with any amount of elements. This isn't possible
with rust functions as they always have a set number of arguments.

> [ Diagram of repetition and transcription? ]

macro_rules allows us to do this by using *repetition*
Repetition allows us to collect up multiple tokens as one argument in the pattern,
then expand it back out again in the body.
The syntax for this can get a little confronting so hang in there!

> [ Example of pattern with * repetition and how it would be called ]

To create a repetition, we surround a part of our pattern with a dollar sign parenthesis and then follow
it with a special symbol. If you've worked with regular expressions then this symbol may be familiar.

We can use an asterisk as this symbol (*) to indicate 0 or more of the inner pattern.

> [ Example of pattern with + repetition and how it would be called ]

We could also use a plus (+) to indicate 1 or more repetitions.

> [ Example of pattern with ? repetition and how it would be called ]

Or a question mark (?) to allow for 0 or 1 repetitions.

> [ Repetition in the transcriber (body) ]

We can then use the same syntax in the body of our macro rule to expand the argument back out.

Note by the way, that like before, the choice of the comma in this repetition is somewhat arbitrary.
we could use another symbol or none at all.

Lets use this for our `assert_gt` macro!

## Macro Example 2 - Variadic assert_gt

> [ showing how we can call assert_gt with multiple arguments ]

We want it to work like this, where each element must be greater than the next.

(...adding a new rule, talking about recursion (with a diagram), etc


