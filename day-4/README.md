# Day 4: Printing Department

Link: <https://adventofcode.com/2025/day/4>

>[!NOTE]
>Hi everyone! This is Day 7 Theodore speaking (or writing I guess). I was
>incredibly busy on Day 4 due to school, but I am back to defeat the challenges
>I missed while I was gone!

## Part 1

### Problem (1)

It's time for grids adjacent cells, everyone! Our input is very simple this
time, consisting of only periods (`.`, `U+002E`) and at-symbols
(`@`, `U+0040`), which make up a grid.

For each at-symbol, we need to count the number of adjacent cells that are
*also* at-symbols. If there are only 3 or less at-symbols in the eight adjacent
cells, we should increment a counter which we return at the end.

>[!NOTE]
>If an at-symbol is at the edge of the grid, we can assume that the grid edges
>are also made out of periods.

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
3. For each at-symbol, check if adjacent cells have <4 at-symbols
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

113 SLoC according to [`scc`](https://github.com/boyter/scc). I mean, it's very
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

### Steps to solve (2)

TODO

#### Time it took me to solve (2)

I haven't solved it yet!

#### Source Lines of Code (2)

I haven't solved it yet!
