# Macros Rule

## Theming

- medieval/kingdom aesthetic
- a little bit wacky

## Ideas

what if each example is a decree from the macro ruler?

- decree 1: raise taxes
- decree 2: perform a census
- decree 3: all tax calculations must be performed with macros

## Sections

- Intro to macros
- Discuss how macros are made
- Example 1 -> repetition
- Reasons for making macros
- Example 2 -> variadics
- Example 3 -> DSL?


## Plan

--- intro to macros ---

these are macros, they expand into rust code

-> brief intro to macros
-> lets make our own!

--- lets make our own! ---

two main ways,
- macro_rules! -> "write a series of substitution patterns"
- proc macros -> "write a rust program"

"But, you'vve seen the name of the talk... Lets talk about macro_rules!"

---- motivating example 1 ----

testing some code in a repetitive way.
make it medieval themed -> tax calculation?

---- reasons for macros ----

Common reasons for macros
reduce repetition (we just saw)
“function-like” behaviour with flexible arguments (variadics)
DSLs

lets investigate variadics

---- motivating example 2 ----

(variadics)

---- motivating exampple 3 ----

terrifying s-expression macro

---- thanks for listening! ----
