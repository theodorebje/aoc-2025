# Day 1: Secret Entrance

Link: <https://adventofcode.com/2025/day/1>

## Part 1

### Problem (1)

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

### Steps to solve (1)

1. Implement wrapping dial
2. Implement command
3. Parse input
4. Count number of times the dial points at 0.

#### Time it took me to solve (1)

1 hour, 6 minutes, 1 seconds (01:06:01). Honestly not very proud of myself on
this one. The `Sub` method on `Dial` was the most difficult part, and took so
long! In the end, I didn't even write it in idiomatic Rust code. I left a TODO
comment in `src/dial.rs` for that. Let's hope Part 2 is easier!

#### Source Lines of Code (1)

174 SLoC according to [`scc`](https://github.com/boyter/scc).

## Part 2

### Problem (2)

We were actually supposed to count the number of times that the dial passes 0.
This means that e.g. L100 will always pass zero once. This is great for my
`Sub` implementation, as it means that it will be incredibly easy to adapt it to
these new rules. It won't be as easy for the `Add` implementation, though, as
that uses modulo.

I want to go to bed, so I am going to solve this as naïvely as possible.

### Steps to solve (2)

1. Rewrite the `Add` implementation to mirror that of the `Sub` implementation
for simplicity
2. Have like a static, global `LazyCell` or something where we count the number
of zeroes naïvely.

#### Time it took me to solve (2)

10 minutes, 15 seconds (00:10:15). I left the codebase in a way worse-off
position than before, but I'm tired and this means that I get to go to bed, so I
consider this an absolute win.

#### Source Lines of Code (2)

192 SLoC according to [`scc`](https://github.com/boyter/scc).
