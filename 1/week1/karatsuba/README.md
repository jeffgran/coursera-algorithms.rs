# Karatsuba Multiplcation

[Wikipedia Page](https://en.wikipedia.org/wiki/Karatsuba_algorithm)

This is a multiplication algorithm that runs in O(n^1.585), rather than the naive implementation which runs in O(n²) where n = the number of digits in the input numbers.

The intention was to be able to efficiently compute products of numbers of any length (i.e. not limited by Integer::MAX).

To that end, I implemented a custom `Digits` struct which wraps a list of digits of any length. I implemented Add, Subtract and of course Multiply for this struct.


usage:

```
$ cargo build
   Compiling karatsuba v0.1.0 (file:///Users/jeffgran/dev/rust/coursera-algorithms/1/week1/karatsuba)
   Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
$ ./target/debug/karatsuba 700805354062759011160628710135740046652 1234123412341234123456785678567856785678
700805354062759011160628710135740046652 * 1234123412341234123456785678567856785678 = 864880294942938913694729091151167344758624285026898142920969972015586485450056
```


### Known issues

This only works with two inputs of roughly the same size. Because I'm lazy and didn't figure out how to handle the base case where you have 1-digit times many-digits without overflowing the native integer type.
