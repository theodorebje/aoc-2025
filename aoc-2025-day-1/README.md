# Day 1: Secret Entrance

Link: <https://adventofcode.com/2025/day/1>

## Problem

We have a dial with 100 numbers, 0 through 99. When we go below 0, we should
wrap around to 99, and when we go above 99, we should wrap around to 0.

The dial takes commands, separated by newlines. Each command consists of a
direction and a number, like this:

```txt
L50
R2
L59
R40
L0
R2
```

>[!NOTE]
>The commands for the problem exists at [input.txt](./input.txt) (not commited
>to the repository,
>[get your own copy](https://adventofcode.com/2025/day/1/input)).

L stands for "Decrease", while R stands for "Increase". The numbers are the
amount we should increase or decrease by, (remember to wrap around).

The dial starts at 50.

**Count the number of times that a command results in dial showing a zero.**
That is the password (answer).

## Steps to solve

1. Implement wrapping dial
2. Implement command
3. Parse input
4. Count number of times the dial points at 0.

## Solution

### Time it took me to solve

I haven't solved it yet!

### Source Lines of Code

I haven't solved it yet!
