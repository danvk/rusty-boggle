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

[1]: https://github.com/danvk/performance-boggle
[2]: http://www.danvk.org/wp/category/boggle/
