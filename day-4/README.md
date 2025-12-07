# Day 4: Printing Department

Link: <https://adventofcode.com/2025/day/4>

>[!NOTE]
>Hi everyone! This is Day 7 Theodore speaking (or writing I guess). I was
>incredibly busy on Day 4 due to school, but I am back to defeat the challenges
>I missed while I was gone!

## Part 1

### Problem (1)

It's time for grids adjacent cells, everyone! Our input is very simple this
time, consisting of only periods (`.`, `U+002E`) for empty space and at-symbols
(`@`, `U+0040`) for rolls of paper, which make up a grid.

For each roll of paper, we need to count the number of adjacent cells that are
*also* taken up by a roll of paper. If there are only 3 or less rolls of paper
in the eight adjacent cells, we mark the targeted roll of paper as "accessible",
and we increment a counter which we return at the end.

>[!NOTE]
>If a roll of paper is at the edge of the grid, we can assume that the grid
>edges are also made out of empty space.

Here's an example input:

```txt
.@.@.@@.@.@..
@.@.@...@@..@
..@@...@.@.@.
.@.@.@@.@.@..
@.@.@...@@..@
..@@...@.@.@.
```

>[!NOTE]
>The commands for the problem exists at [input.txt](./input.txt) (not commited
>to the repository,
>[get your own copy](https://adventofcode.com/2125/day/2/input)).

I am going to try to be a little bit quicker than on day 3, because I really
want to finish more than one challenge today.

### Steps to solve (1)

1. Implement grid
2. Implement way to get adjacent cells to cell
3. For each roll of paper, check if adjacent cells have <4 rolls of paper
4. If so, increment counter

#### Time it took me to solve (1)

Remember what I said about being a little bit quicker today? Let's get an
instant replay of what I said:
>I am going to try to be a little bit quicker than on day 3, because I really
>want to finish more than one challenge today.
>
>―Theodore, a few hours earlier

Yeah, that didn't work out.

One. Hour. Fourty-four. Minutes. Fifty-eight. Seconds.

this was ~~a bit~~ **WAY** more challenging than I expected

Debugging this challenge made up roughly 90% of the time I spent on this
problem, I would say. I hated it. It even made me whip out GDB, and I haven't
used that thing in over a year. I had to lookup the man page for it.

#### Source Lines of Code (1)

119 SLoC according to [`scc`](https://github.com/boyter/scc). I mean, it's very
readable. But that's not a lot of lines for how long I spent on this problem.

## Part 2

### Problem (2)

Speedrun time! I am going to try to get sub-10 minutes on this one. I have a
sneaking suspicion that I will regret saying that, but what is Advent of Code
without some self-imposed rules? I need some sort of punishment for if I fail,
though… Hmm, no sugar for a day. That won't feel too punishing if I through
inexplicable means somehow fail??

>Don't jynx it!1!1!!
>
>―Theodore from the future (probably, this is still present me)

Okay, so we need to remove the rolls that we determined are accessible, and then
repeat the algorithm again until there are no more accessible rolls of paper.

Instead of counting the number of accessible rolls of paper, we are supposed to
count the total number of rolls of paper that we have removed from the grid.

I can already feel that this is definitely not going to take less than 10
minutes, but you can always hope, I guess?

### Steps to solve (2)

1. Remove accessible rolles of paper
2. Count how many you removed
3. Feed the new grid into accessible-roll-remover algorithm again
4. Repeat

#### Time it took me to solve (2)

NOOOOO I JYNXED IT!!! IT TOOK 16 MINUETES AND 42 SECONDS.

the worst part is that I solved the problem somewhere around the 7 minute mark,
but I got hit in the face with the infamous
[`E0502`](https://doc.rust-lang.org/error_codes/E0502.html):

```rust
error[E0502]: cannot borrow `*self` as immutable because it is also borrowed as mutable
  --> day-4/src/part_2/mod.rs:76:45
   |
73 |         for (y, row) in self.0.iter_mut().enumerate() {
   |                         -----------------------------
   |                         |
   |                         mutable borrow occurs here
   |                         mutable borrow later used here
...
76 |                     let count = Cell::rolls(self.adjacent(x, y));
   |                                             ^^^^ immutable borrow occurs here
```

I rarely use mutable borrows in the Rust code that I personally write, so I
wasn't immidiately sure how to solve the problem. Once I had finally fixed it,
it was already too late. I had failed my sub-10 minute challenge.

Why did I set it at sub-10??? It would have been slightly more possible with
sub-15 (I stopped trying a bit once it passed the 10-minute mark). Sub-15 would
still have been a challenge.

*Sigh*. I'll leave you with this quote.

>>Don't jynx it!1!1!!
>>
>>―Theodore from the future (probably, this is still present me)
>
>Indeed, past me did jynx it.
>
>―Real Theodore from the future

#### Source Lines of Code (2)

101 SLoC according to [`scc`](https://github.com/boyter/scc). Also quite
readable. Overall, I think I left myself a very solid foundation for myself with
part 1's code, I just didn't possess enough mutable Rust knowledge to harness
that solid foundation in time.
