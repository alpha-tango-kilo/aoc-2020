# Day Seven - Handy Haversacks

[**Link**](https://adventofcode.com/2020/day/7)

## Part one

I have nothing against recursion except that it's a pain to debug if you don't get it right first time.
I did not get it right first time.

I tried to shortcut my solution by using numbers that were returned if a bag contained another specific colour of bag.
This ended up being a bad idea as multiple branches of the same tree could contribute more than one solution (e.g. red can contain green which can contain gold, **and** red can contain yellow which can contain gold).
Boolean OR was the correct answer here so I changed my recursive function signature to that and the branching code applies OR using the `.any()` iterator consumer
