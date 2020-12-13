# Day Four - Passport Processing

[**Link**](https://adventofcode.com/2020/day/4)

## Part one

More string parsing, nothing wrong here.
I was going to make a solution that had a struct with an `Option<String>` for every possible entry, but I hoped that wouldn't be necessary for the second part and did a map instead, far easier!

## Part two

Nothing tough about this one for me, just a lot of tedium choosing regexes and writing the match statements.

Was interesting to mess around with Rust's iterator `all()` and `any()` methods, as well as using `lazy_static!` for having statics that could be made using non-constant functions
