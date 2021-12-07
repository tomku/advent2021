Hello! This repository contains my solutions for [Advent of Code 2021](https://adventofcode.com/2021), a yearly event
where a pair of seasonally-themed programming puzzles are released for each of the first 25 days of December.

Some people compete to solve the problems as quickly or as efficiently as possible. Some (like me) use the problems as
exercises to learn or solidify understanding of a new programming language. I'm solving this year's puzzles
in [Rust](https://www.rust-lang.org).

Since the goal is to learn new things, some of the solutions might be inefficient or contain newbie mistakes and that's
ok! I don't need issues filed about things I could improve, as optimized solutions for Advent of Code are usually not
very difficult to find online should I want them.

I've chosen to structure this year's solutions as a set of unit tests. You can run them by typing `cargo test`, but as
they produce no output it's probably more interesting to look at the tests themselves, which contain both the answers
reached (as literals in `assert_eq!`s) and invocations of the code that calculates those answers.