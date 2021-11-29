# rsj - a toy implementation of J in Rust

[J](https://www.jsoftware.com/help/dictionary/intro.htm) is a strange but
fascinating programming language descended from APL: 

* Idiomatic J uses no loops or control flow statements, and not even any
  explicit functional applications like `map` or `filter`.

* Language components are named after English parts of speech: nouns (objects),
  verbs (functions), pronouns (variables), etc.

* All primitive verbs (like built-in functions) are named with one or two
  punctuation characters. Instead of `len` we have `#`. (This is however a
  considerable concession to reality from APL, which used non-ASCII punctuation
  and mathematical symbols that many people may not even know how to pronounce.)

* All verbs can be applied to either a single argument or, in infix style, to
  two arguments. In other languages we see this with minus in `-a` vs `a-b` but
  J uses it everywhere.

* No operator precedence or associativity.

* Verbs can be composed with *adverbs* or *conjunctions* to use new verbs.

I thought it would be fun to understand J better by writing an interpreter in Rust.
