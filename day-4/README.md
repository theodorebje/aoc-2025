# Day 3: Printing Department

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

I haven't solved it yet!

#### Source Lines of Code (1)

I haven't solved it yet!

## Part 2

### Problem (2)

TODO

### Steps to solve (2)

TODO

#### Time it took me to solve (2)

I haven't solved it yet!

#### Source Lines of Code (2)

I haven't solved it yet!
