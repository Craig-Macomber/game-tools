# Dicey

A dice roller library written in Rust.

This crate aims at providing everything needed for playing tabletop RPGs.

This is a fork of [caith](https://github.com/Geobert/caith) used by [https://inland.martials.org/](https://inland.martials.org/).

## Syntax

_x_ d _y_ [OPTIONS] [TARGET] [REPETITION] [! REASON]

Roll _x_ dice(s) with _y_ sides.

`y` can also be "F" or "f" for [fudge dice](https://en.wikipedia.org/wiki/Fudge_%28role-playing_game_system%29#Fudge_dice) which are either `[−]`, `[+]` or `[ ]` (treated as -1, 1 and 0 when used numerically).
For example `3dF` might yield a roll of `[( ), (-), (-)] = -2`

When grouping is unclear of needs to be adjusted, parentheses can be used.
For example `(1d4 + 2d6) / 2` (and produces a roll like `([2] + [2, 5])/2 = 4.5`).

### Options:
- `+` `-` `/` `*` : modifiers. `1+2-3/4*5`, `2d6+1d4+4`
- `e`_#_ : Explode value. If number is omitted, we use the maximum value of the dice. `1d4e` will add a second `d4` if it rolls 4. `1d4e3` will add a second dice on 3 or higher.
- `ie`_#_ or `!`_#_ : Indefinite explode value, If number is omitted, we use the maximum value of the dice.`1d4ie3` or `1d4!3` will keep adding dice until a value lower than 3 is rolled.
- `K`_#_  : Keeping # highest (uppercase "K"). `2d20K1` keeps the highest of two `d20` rolls.
- `k`_#_  : Keeping # lowest (lowercase "k"). `2d20k1` keeps the lowest of two `d20` rolls.
- `D`_#_  : Dropping the highest (uppercase "D"). `2d20D1` drops the highest of two `d20` rolls (same as keeping the lowest in this case).
- `d`_#_  : Dropping the lowest (lowercase "d"). `2d20d1` drops the lowest of two `d20` rolls (same as keeping the highest in this case).
- `r`_#_  : Reroll if <= value. `8d6 r1` will roll `8d6` but reroll any ones.
- `ir`_#_ : Indefinite reroll if <= value. `8d6 r1` will roll `8d6` but reroll any ones, including any rolled during the reroll, indefinably.

### Target:

An expression may optionally specify targets using any combination of at most one of these options.
An expression which uses any of these will report the number of successes minus the number of failures instead of the numeric value rolled.

- t#  : minimum value to count as success. `2d6 t3` will roll two six-sided dice, and return how many were 3 or higher.
- tt# : minimum value to count as two successes. `2d6 t3 tt6` will roll two six-sided dice, and return how many were 3 or higher, double counting any sixes.
- t[<list of numbers>] : enumeration of values which count as success. `3d6 t[2,4,6]` will count the listed even numbers as success (handy for games like "Knight").
- f# : value at of below which is considered as failure. Failures are counted as `-1` successes.

Combinations like `2d6 f1 t3 tt6` are supported.

## Commands

An `Expression` (using the above features) defines a roll which produces a numeric result.
An `Expression` can be used as a `Command`, but commands can also be extended with:

### Repetition:
A roll can be repeated with `^` operator.

- `^`_#_ : Repeat the roll _#_ times, summing the result. `(2d6 + 6) ^ 8` will roll eight times the expression.
- `^+`_#_ : Repeat the roll _#_ times, summing the result. `(2d6 + 6) ^ 8` will roll eight times the expression, and sum the results.
- `^#`_#_ : Repeat the roll _#_ times, summing the result. `(2d6 + 6) ^# 8` will roll eight times the expression, and sort the results.

### Reason:
- `:` : Any text after `:` will be a comment. `1d6 : Damage` will label the roll as a "Damage" roll.

## Limitations

To avoid out of memory issues, there is a limit of 5000 dice.
For example `1d5001` is will error.
Expressions which produce over 5000 re-rolls `1d100000ir99999` can report `Exceed maximum allowed number of dice (5000) during rerolls.` as well.

## Examples

Some of these examples are directly taken from DiceMaiden's Readme:

`2d6 + 3d10` : Roll two six-sided dice and three ten-sided dice.

`3d6 + 5` : Roll three six-sided dice and add five. Other supported static modifiers are
add (+), subtract (-), multiply (*), and divide (/).

`3d6 * 1.5` : Roll three six-sided dice and add 50%.

`3d6 e6` : Roll three six-sided dice and explode on sixes. Some game systems call this 'open
ended' dice. If the number rolled is greater than or equal to the value given for this option,
the die is rolled again and added to the total. If no number is given for this option, it is
assumed to be the same as the number of sides on the die. Thus, '3d6 e' is the same as '3d6 e6'.
The dice will only explode once with this command. Use `ie` for indefinite explosions.

`3d6 ie6` or `3d6!` : Roll three six-sided dice and explode on sixes indefinitely within reason.
We will cap explosions at 100 rolls to prevent abuse.

`3d10 d1` : Roll three ten-sided dice and drop one die. The lowest value will be dropped first.

`3d10 K2` : Roll three ten-sided dice and keep two. The highest value rolled will be kept.
Using lowercase `k` will keep the lowest.

`4d6 r2` : Roll four six-sided dice and reroll any that are equal to or less than two once.
Use `ir` for indefinite rerolls.

`4d6 ir2` : Roll four six-sided dice and reroll any that are equal to or less than two (and do
the same to those dice). This is capped at 100 rerolls per die to prevent abuse.

`6d10 t7` : Roll six ten-sided dice and any that are seven or higher are counted as a success.
The dice in the roll are not added together for a total. Any die that meets or exceeds the
target number is added to a total of successes.

`5d10 t8 f1` : f# denotes a failure number that each dice must match or be beneath in order to
count against successes. These work as a sort of negative success and are totaled together as
described above. In the example roll, roll five ten-sided dice and each dice that is 8 or higher
is a success and subtract each one. The total may be negative. If the option is given a 0 value,
that is the same as not having the option at all thus a normal sum of all dice in the roll is
performed instead.

`5d10 t8 tt10` : 8 and 9 are counted as success, 10 are counted twice.

`3d6 t[2,4,6]` : only even result will count as success (handy for games like "Knight").

`4d10 k3` : Roll four ten-sided dice and keep the lowest three dice rolled.

`4d6 : Hello World!`: Roll four six-sided dice and add comment to the roll.

These commands can be combined. For example:

`10d6 e6 K8 + 4` : Roll ten six-sided dice, explode on sixes and keep eight of the highest rolls
and add four.

`6d6 e5 K3 r2 d2 e6 + 5df k4 e( )` : Complex expressions like this can have their results rendered to explanatory markdown which details all the steps:
> \[**5**&#x200B;🡵2, **5**&#x200B;🡵2, 1, **6**&#x200B;🡵1, **6**&#x200B;🡵1, 2\]e5 🡲 \[5, ~~*2*~~, ~~*5*~~, ~~*2*~~, ~~*1*~~, 6, ~~*1*~~, 6, ~~*1*~~, ~~*2*~~\]K3 🡲 [5, 6, 6]r2 🡲 \[~~*5*~~, 6, ~~*6*~~\]d2 🡲 \[**6**🡵3\]e6 + \[(-), ~~*(+)*~~, ( ), ( ), ( )\]k4 🡲 \[(-), **( )**&#x200B;🡵(-), **( )**&#x200B;🡵(-), **( )**&#x200B;🡵(+)\]e( ) = **7**
