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

## Supported features

Only floating-point numbers and 1-dimensional arrays of numbers are implemented so far.
(They're actually complex numbers internally but there is no syntax to create complex
numbers yet.)

Monadic and dyadic verb application.

Verbs:

* `-` (minus, negative)
* `-.` (not)
* `#` (tally)

For examples see the `t/` directory: the lines indented by three spaces are the input 
and the unindented lines are the expected output. These are all checked by `cargo test`.

## Goals

* Be able to run any solutions I can write to Advent of Code.

* Be reasonably faithful to the J specification: this is an implementation of J,
  not just a J-inspired language. But, it's not necessary to produce precisely 
  byte-for-byte identical output especially with regard to formatting floats and
  error messages.
  
* Run all examples from the J documentation and tutorials for the features that
  are implemented.
  
* Embrace its terseness and archaic feeling e.g. in error messages and interpreter
  prompts.

* But also add some modern conveniences that don't impinge on the core experience,
  e.g. readline editing.
  
* Support a kind of literate programming by running examples from inside Markdown
  docs (not supported yet) or `.ijs` files.
  
* Maybe, support a notebook interface.
  
* Write clean idiomatic Rust. Understand a good mapping from J types to Rust types.

* Never panic.

## Background reading

* [An Implementation of J](https://www.jsoftware.com/books/pdf/aioj.pdf), by
  Roger Hui, describes the original implementation.
