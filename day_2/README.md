# Day Two - Password Philosophy

[**Link**](https://adventofcode.com/2020/day/2)

## Part one

As much as it's tedious, parsing stupid string formats into something meaningful has never been an issue to me.
I do hate how it looks in Rust code though.
I tried to keep things idiomatic by using the `FromStr` interface

Regex might be a bit bloated for this task but I'm comfy with them

## Part two

I was very upset when I saw part two initially, it looked very different to part one.
I took it as basically a completely independent challenge to part one, completing the challenge but realising how horribly duplicated the code was.
So I proceeded to wrap my head around traits and trait bounds in order to reduce the redundancy down to as close to nil as possible
