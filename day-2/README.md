# Day 2: Gift Shop

Link: <https://adventofcode.com/2025/day/2>

## Part 1

### Problem (1)

We are given a single line containing multiple inclusive ranges separated by commas. Each
range contains two parts, a `FROM` value and a `TO` value, separated by a dash
(`U+002D`, `-`). Our goal is to find which ranges are valid or invalid.

Loop through each value in each range (e.g. for 1-3, you would loop through
[1,2,3]). Split each value in "half", by separating the n/2 first digits (where
n is the digit length of the number) and checking if they are the same as the
other half. Check all of the values represented by the range, as there may be
multiple invalid values represented in a single range. At the end, add up all of
the invalid values and print out the sum.

There are never any leading zeroes, regardless of validity.

Example input:

```csv
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
```

>[!NOTE]
>The commands for the problem exists at [input.txt](./input.txt) (not commited
>to the repository,
>[get your own copy](https://adventofcode.com/2025/day/2/input)).

Unfortunately, I started working on this problem really late, so I am going to
try to speedrun this problem. I am predicting that my code is going to look
terrible, but honestly, I don't have the energy to code properly today. Maybe
I'll revisit my solutions later?

### Steps to solve (1)

1. Parse input
2. Loop through each range
3. Convert value to string
4. Split
5. Check if repeated
6. Add to vector
7. Calculate the sum
8. Print it!

#### Time it took me to solve (1)

9 minutes and 9 seconds (00:09:09). It really shows in the code that I was
trying to just pump out a working solution as quickly as possible, although I
was trying to stay calm and collected to be able to false asleep better whenever
I finish this puzzle.

#### Source Lines of Code (1)

25 SLoC according to [`scc`](https://github.com/boyter/scc).

I also found these stats quite amusing:

```txt
Estimated Cost to Develop (organic) $561
Estimated Schedule Effort (organic) 0.80 months
Estimated People Required (organic) 0.06
```

If anyone's interested in sending over $561 for full ownership rights of these
fine 25 lines, hit me up! My email is in the git commit message.

## Part 2

### Problem (2)

We can no longer use our "split in half" solution, because we now want to check
if a value has any number of repeating characters. Previously, 111 would be a
valid value, but now we have to reject that. We also have to reject e.g. 121212.

How do we solve this? No idea. Yes, I am typing this out as I am trying to solve
the problem in my head. Maybe there's some standard library way to do this
(probably not). Let me check...

No, of course not. Why would there be one?

I genuinely can't think of a good solution for this problem, and I'm not sure
whether that is because it's a difficult problem or because I am extremely
tired. Probably both.

My hacky solution (please don't copy this solution if you are reading this and
looking for solutions): brute force all possible combinations.

### Steps to solve (2)

(Continuing from step 3 on part 1):

1. Loop through the length of the string
2. Split it `j` times.
3. Check if all splits are equal.
4. If not, move on to next length.

#### Time it took me to solve (2)

I haven't solved it yet!

#### Source Lines of Code (2)

I haven't solved it yet!
