# Day 5: Cafeteria

Link: <https://adventofcode.com/2025/day/5>

>[!NOTE]
>Hi everyone! This is Day 5 Theodore speaking! Sorry, you're confused why
>[day 3](../day-3/README.md) and [day 4](../day-4/README.md) were made in the
>future..?
>
>…
>
>Oh! You're asking how it's possible that they were made on day 6 and 7 (just a
>*tiny* bit late), respectively, if this one was made on the correct day
>(day 5)? No, no! That's not the case at all. Today is day 5 of *January*
>2026!!! You might be wondering why: wellll, let's just say I had a bit to do in
>December…
>
>How's 2026 so far? Well, it certainly
>[kicked off with a bang](https://www.france24.com/en/americas/20260103-multiple-explosions-aircraft-sounds-reported-in-venezuela-caracas-maduro-trump),
>so to speak…

## Part 1

### Problem (1)

We are given two lists, separated by a blank line. The first list contains a
bunch of inclusive ranges of the IDs of fresh ingredients. Note that the ranges
can overlap. The list of ranges is not sorted.

The second list contains the ingredient IDs that we are working with. Our job is
to count how many ingredients are fresh. Note that the list of ingredient IDs is
not continuous, e.g. it can go from 1 to 3, skipping ingredient #2. The list of
ingredient IDs are sorted.

Here's an example input:

```txt
1-3
7-9
5-8
24-42

1
3
7
12
36
48
```

>[!NOTE]
>The commands for the problem exists at [input.txt](./input.txt) (not commited
>to the repository,
>[get your own copy](https://adventofcode.com/2025/day/5/input)).

The above example input should give us an output of `4`, if I have counted
correctly (I really hope I have, otherwise I'll waste a lot of time debugging
for nothing)…

I suspect that this will be an extremely easy challenge.

### Steps to solve (1)

1. Parse both
2. hashset
3. hashset
4. hashset
5. hashset
6. Done

#### Time it took me to solve (1)

TODO

#### Source Lines of Code (1)

TODO

## Part 2

### Problem (2)

TODO

### Steps to solve (2)

TODO

#### Time it took me to solve (2)

TODO

#### Source Lines of Code (2)

TODO
