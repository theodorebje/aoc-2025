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
ingredient IDs is ~~sorted~~ **NOT** sorted.

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

16 minutes and 26 seconds. Easy-peasy-lemon-squeezy.

>While searching for if I spelled that idiom correctly (I did!), I found out
>that you can
>[apparently say "Easy-peasy-Japanesey" instead](https://idiomorigins.org/origin/easy-peasylemon-squeezyjapanesey),
>which I found very interesting, and I am definitely going to start saying that
>from now on! The Japanesey version is apparently an Aussie idiom too! This
>idiom only gets better and better!! I should start using idioms more often…

It took me a little longer than I would have liked to, simply because I was
debugging why this line didn't work:

```rust
if fresh.0.iter().any(|range| matches!(ingredient, range)) { /* … */ }
```

This simply returned `true` for each and every element in `fresh`, which seemed
like a bug in the `matches!` macro to me at the time, but now that I have the
time to sift through the code, I realise that it just expands to this:

```rust
if fresh.0.iter().any(|range| {
    match ingredient {
        range => true,
        _ => false,
    }
}) { /* … */ }
```

If you know anything about the `match` syntax, chances are that you immediately
see the problem at hand. Let me show you another use of match to make it even
more obvious:

```rust
match binary {
    0 => false,
    1 => true,
    other => panic!("Not binary: {other}"),
}
```

Effectively, both of the match arms have become
[catch-all's](https://doc.rust-lang.org/book/ch06-02-match.html#catch-all-patterns-and-the-_-placeholder),
with the only difference being that the first arm assigns the ingredient to the
`other` binding, which is immediately dropped.

For anyone wondering, the way're supposed to check if a number is in a range
that's stored in a variable is:

```rust
if fresh.0.iter().any(|range| range.contains(&ingredient)) { /* */ }
```

#### Source Lines of Code (1)

46 SLoC according to [`scc`](https://github.com/boyter/scc). And I would say
that it's 46 lines of very readable and pretty code too. I'm proud of my work
here. Let's see if I'll still be proud at the end of part 2.

## Part 2

### Problem (2)

Now we are going to ignore the second list, and just count the max number of
fresh ingredients possible.

I'm going for sub 30 seconds on this one.

### Steps to solve (2)

1. Just count it?

#### Time it took me to solve (2)

Wow. Just wow. I was absolutely scammed out of my time. Let me retell this story
to you before I show you how long it took.

>Hey, Theodore from the future here. *I wrote all of this below me???* Holy moly
>what a rant. I'm putting this in a collapsible section. Open at your own risk
>
>―Theodore from the future

<details>

<summary>Yap yap yap yap</summary>

Okay, so it might be true that I jynxed it. I don't think I really understood
what the problem was about? It took about 8 minutes (IIRC) to create the initial
version.

The funny thing is, if you compare that version that I wrote after 8
minutes with the version that I have right now, you'll notice that only a few
lines changed (why am I saying notice? I don't even have the original version stored
anywhere except for in my head). In fact, IIRC, I can count the number of lines
changed on one hand.

How long can it take to change, what, max 5 lines of code?????

Apparently very.

So, I may or may not have thought that there couldn't be any duplicate range
starts. My thoughts were incorrect. There were duplicate range starts. "Why is
that bad?", you may ask. I was storing the range starts in a
[BTreeSet](https://doc.rust-lang.org/nightly/std/collections/struct.BTreeSet.html)
while prototyping the whole thing. There can't be duplicates keys a
`*Map` / `*Set`. But Rust is fail-fast, right? So, of course I would realise my
mistake pretty quickly and fix it right away. That is, if it had failed fast.
This is the method signature of
[`BTreeSet::insert`](https://doc.rust-lang.org/nightly/std/collections/struct.BTreeSet.html#method.insert):

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_confusables("push", "put")]
pub fn insert(&mut self, value: T) -> bool
```

It returns a boolean. And there is no
[`#\[must_use\]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute)
attribute on booleans. And the attribute is not present on the insert method
either. Since I was trying to prototype as quickly as possible, I didn't notice
that I had to check the return value!!! And yes, this is my official request to
the Rust `std` library maintainers; **please** mark
[`BTreeSet::insert`](https://doc.rust-lang.org/nightly/std/collections/struct.BTreeSet.html#method.insert)
as
[`#\[must_use\]`](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute)!

I kind of went mad trying to debug why my code wasn't
working correctly. All of my tests passed, because the example ranges on the AOC
page say *NOTHING* about duplicate range starts, only overlapping ranges (which
I was handling properly). But it's just the examples that don't show every
edge-case, right? Surely, the actual problem text on the AOC page mentions the
duplicate range starts, right? Right???? NO!! I was up until 1 AM trying to
debug this piece of `꧁✬◦°⋆⋆°◦. schite ◦°⋆⋆°◦✬꧂` (pardonen mi Middel
Englisch) (sorry that was very cringe, I regret typing that out).

If it wasn't obvious, I'm just a tiny bit frustrated…

</details>

Oh, look at the time! No, not the clock on the wall! The time it took me to
code together this! Or rather debug it, because most of the the time it took me
to finish this was just debugging.

If you count me sleeping (yes, I did give up at 1 AM and hit the sack) and doing
other non-productive stuff, I estimate it took me ~18 hours (I don't know, I
didn't count my snores). If you don't count my highly productive snoring, it
took one hour, 42 minutes, and 32 seconds.

🙂

#### Source Lines of Code (2)

At the height of my debugging frenzy, I was at 126 SLoC according to
[`scc`](https://github.com/boyter/scc). With my code properly cleaned up, I am
at 98 SLoC. I don't remember if I counted tests for the other days, but if you
don't count tests, it's 67 SLoC.
