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

So, slight problem… I took a quick little break, and I might have – how do I put
this…? – forgotten to turn the stopwatch back on? I genuinely have no idea how
long it took, so I am just going to round it to an hour and leave a nice
footnote in the statistics. Let's hope I don't mess up with the stopwatch again
in the future!

#### Source Lines of Code (1)

Now this is data that I do actually have! 54 SLoC according to
[`scc`](https://github.com/boyter/scc). And I am actually quite proud of the
code I wrote this time, although it took a good while to debug.

## Part 2

### Problem (2)

¡We need more power! Instead of the two batteries per bank required from part 1,
we now need a whopping **12** per bank!!! I'll have to completely rethink how to
structure my code, because I had hardcoded the two batteries per bank into my
code, and hardcoding 12 batteries will be unsustainable.

### Steps to solve (2)

1. Store an array of batteries instead
2. Find the twelve largest numbers
3. Add them together!

#### Time it took me to solve (2)

1 hour, 4 minutes, 19 seconds (01:04:19). Part 2 was really difficult, as it
involved refactoring most of the code. It was really fun, though! And as a
cherry on the top, I got to learn about the `fold` method, which I had never
used before. The code is actually way more readable than I thought it would turn
out. Somehow I also resisted the urge to just write a macro, too, so that's
cool, I guess.

#### Source Lines of Code (2)

135 SLoC according to [`scc`](https://github.com/boyter/scc). A tad longer, but
still surprisingly readable. I have no guilt at all with putting my name behind
this code, even though there is some unsafe thrown in there. But, hey, we need
those performance gains, right!? (ignore the fact that I don't even have a way
to benchmark my solutions yet lol)
