# Day Eleven - Seating System

[**Link**](https://adventofcode.com/2020/day/11)

## Part one

Given how long it's been, I was definitely feeling rusty going into this.
I'm happy enough with the situation, though this definitely took more fighting with the borrow checker than I expected.
A couple niggly details in the prompt caught me out - such as having to apply all changes in one go, but overall I'm fairly happy with my solution, though it's maybe more memory intensive than I'd like

## Part two

We don't talk about this.
I hate everything about `Direction`, if only I could just do `usize += i8`, none of this would have happened!
Instead I now rely on integer overflow, just perfect.
I even ended up implementing `Display` twice just so I could try and work out what was going on for my other troubles 😫
