# Day One - Report Repair

[**Link**](https://adventofcode.com/2020/day/1)

## Part one

So for this problem I didn't bother with a sophisticated solution, settling for n squared big O and using two nested for loops.

Probably not the most idiomatic Rust solution either given the lack of iterators, but it'll do

## Part two

Well with my part one solution I failed to guess what they'd make harder for part two (changing the number to be summed to), however it did still end up helping.

Changing some function signatures made return types more useful and allowed for reuse of code I had already written.
Yes my solution is O(n^3) but at the same time I feel like the compiler probably sorted a lot of that out given how fast everything ran
