# Rusty Boggle

Port of [performance boggle][1] to Rust. While I've [known a lot about Boggle][2] at points in the past, I'm a bit rusty.

## Porting

I'm going off the `paper` version of Boggle in performance-boggle, which was intended to be clean.

Trie
Boggler
Perf test

Is `[Option<Trie>; NUM_LETTERS]` as efficient as `Trie*` in terms of memory layout? How can I tell?

The "mark" optimization is going to be trick since all the Tries will need to be mutable.

Are `struct` members public? How does access control work?

What is Rust naming convention? `my_method` or `myMethod`? Is there a preference?

vscode stopped showing errors / autocomplete in a way that makes me uncomfortable.
--> I got no error checking / language services until I imported from `main.rs`.

Initializing this structure was surprisingly hard:

    children: [Option<Box<Trie>>; NUM_LETTERS],

I got a reference to a GitHub issue when I tried `[None; NUM_LETTERS]`!
https://github.com/rust-lang/rust/issues/49147

Unit tests live in the same file as their code, in a `tests` module.
The "if it compiles it works" mantra seems to be holding up well so far.

Is there any way to automatically call `destroy` methods when a structure is consumed?

Is there a pattern for de-duping code between a mut and non-mut method?

It's going to pay off to get very, very comfortable with idiomatic `Option` usage.
- It's not really clear to me when I can use `flat_map`.
- The trailing `?` operator is helpful.
- Is there an equivalent of refinement? i.e. panic on Err

It seems like sometimes `Box` gets implicitly unwrapped in ways that confuse me.
For example, this looks like it should be the identity:

    Some(c) => match c {
        Some(d) => Some(d),
        None => None,
    }

But `c` is `&Option<Box<Trie>>`, so it actually secretly unboxes the Trie.
Is there a shorter way to write this?

First cut:

    $ time ./target/debug/rusty-boggle ../performance-boggle/words
    Loaded 173528 words into 389309 nodes from ../performance-boggle/words
    ./target/debug/rusty-boggle ../performance-boggle/words  1.79s user 0.04s system 99% cpu 1.843 total

I'm a little worried about that performance while loading the Trie.

Wow, writing functions from str -> str is complicated!
https://stackoverflow.com/questions/29781331/why-cant-i-return-an-str-value-generated-from-a-string

Array indexing has been a bit painful. To index into an array, you need a `usize`.
But to subtract one from the x position, you need to do `x - 1`, or `x + (-1)`, which could overflow.
This is all pretty annoying to model out!

I'm confused by how hard it is to import modules in Rust.
I have:
src/
  boggler.rs
  trie.rs
  main.rs

To import trie.rs from boggler.rs, I had to do this:

    #[path = "./trie.rs"]
    mod trie;

But then in `main.rs`, I get this:

    mismatched types
    expected struct `boggler::trie::Trie`, found struct `trie::Trie` rustc(E0308)

As though these weren't the same. Apparently the trick is `crate`:

    use crate::trie;

This _only_ works if you also have `mod trie` in `main.rs`.
This also doesn't work from a file in `bin/prog.rs`.

You can't mutate parameters as you can in C or JS.

The mutable Trie is making me fight with the borrow checker.
This is a big performance win (it means you don't need a cleanup) but I can see why rust thinks it's unsafe.
My workaround is to disentangle ownership of the Boggler & the Trie.
I don't think this is good from a safety perspective.

Unclear if you're allowed to compare pointers for equality in Rust:
https://users.rust-lang.org/t/is-any-way-to-know-references-are-referencing-the-same-object/9716/5
This makes me wonder if choosing such a performance-sensitive application was good for a first project!

`vec.iter().enumerate()` is a useful pattern: iterate over (index, value) pairs.

First indication of performance (debug build, incorrect results):

    Evaluated 216320 boards in 14.541386 seconds = 14876.161 bds/sec

To do an optimized build, you run `cargo build --release`.

The release build is much faster:

    Evaluated 216320 boards in 2.0822232 seconds = 103888.96 bds/sec

But still only ~half the speed of the C++ version:

    Evaluated 216320 boards in 0.929824 seconds = 232646.209370 bds/sec

Some notes from reading about lifetimes and generics in the Rust book:

- In general Rust uses move semantics for complex objects and copy for primitives.
  This is decided by whether a type implements the `Copy` trait.
  The upshot is that there are generally no implicit and expensive operations like in C++.
- Originally, reference parameters _always_ required lifetime annotations.
  To streamline things, there are two special cases:
  1. If there's only one input reference, the output reference gets its lifetime.
  2. Output references get the lifetime of `&self`.
- You usually want to take `&str` as a parameter and return either:
  - `&str` (if it's a slice or the whole input) or
  - `String` if it isn't, which forces the caller to take ownership.

~A BUG! In my 13 year old C++ code, no less. It doesn't find "suqu" on the board "czzdlqzemauopezs".~
~The web UI for this board has many bugs: <https://www.danvk.org/boggle/?board=czzdlquzemauopezs>.~

The word was "suq", not "suqu". So it's not a Boggle word.

## General Notes

- "Classic" C++ is really drowning in type annotations.
- I'm seeing a 3-10x speedup on my 2020 macBook Pro vs. what I recorded ~10y ago:
  $ ./4x4/perf_test
  Evaluated 216320 boards in 0.953688 seconds = 226824.728068 bds/sec
  vs README.md: "88889.509057 bds/sec"
  and 4x4/perf_test.cc: "~20kbds/sec"
- The README references "max+one" but I don't see this in the code.
  Maybe there's a newer version in a repo somewhere?
- Some of the files in the repo are 13 years old.
  I didn't think GitHub was that old; maybe this is an svn import?
- The repo is well-organized; I particularly like the subdirectories with abandoned ideas and short explanations of why they didn't pan out.

Interesting reading this comment with "Don't put type information in the documentation":

    // Assumes ownership of the Trie. No other Boggler may modify the Trie after
    // this Boggler has been constructed using it.
    Boggler(TrieT* t);

From a Rust perspective, that's type information!

[1]: https://github.com/danvk/performance-boggle
[2]: http://www.danvk.org/wp/category/boggle/
