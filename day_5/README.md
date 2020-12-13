# Day Five - Binary Boarding

[**Link**](https://adventofcode.com/2020/day/5)

## Part one

I knew I wasn't going to enjoy this challenge.
Off by one errors are awful and little things like knowing whether to take the `high` or `low` final value when doing binary space partioning trip me up and take way to long to debug and solve.

That having been said, I like the code I've written in the end, making proper use of Rust's traits so it'll do all the heavy lifting for me

## Part two

First semi-confusing prompt, but I did get the right answer so I shan't complain too much.
Ended up being O(n^3) though which sucks but I didn't know a data structure with my three requirements off the top of my head:

* No-duplicates data structure
  
* Fixed order / trivially splittable
  
* Efficient to search

A set did the first and third, but not being able to split meant not being able to enumerate over all possible pairs of seats without duplicates.
A vector did everything just the search is O(n) so yeesh.
A sorted vector could have been better but after getting the right answer I was just happy to be done when I'm already so behind
