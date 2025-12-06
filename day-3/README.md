# Day 3: Lobby

Link: <https://adventofcode.com/2025/day/3>

>[!NOTE]
>Hi everyone! This is Day 6 Theodore speaking (or writing I guess). I was
>incredibly busy on Day 3 due to school, but I am back to defeat the challenges
>I missed while I was gone!

## Part 1

### Problem (1)

We have multiple lines of numbers. Each line represents a `Bank` with a number
of `Batteries`. A battery consists of a single digit, which is its `Joltage`. A
`Joltage` is a number between 1 and 9, inclusive.

Our input looks something like this:

```txt
595684181961911
197153961692818
423959961375935
221141825554245
```

>[!NOTE]
>The commands for the problem exists at [input.txt](./input.txt) (not commited
>to the repository,
>[get your own copy](https://adventofcode.com/2125/day/2/input)).

We have to find the two largest numbers in each bank and concatenate them.
However, we can't simply sort the banks, as the two largest numbers we find
**MUST** be in their original order (e.g. from a string of `12345`, `54` is NOT
the answer, `45` is).

After calculating the max joltage of each individual bank, we receive this
(based on the previously mentioned output):

```txt
99
99
99
85
```

Simply add up these numbers to get our answer.

### Steps to solve (1)

1. Parse the input into a vector of banks.
2. In each bank:
    1. Find the largest non-last number
    2. Find the largest number after the number from step 2.1.
3. Sum up the numbers from each bank.

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
