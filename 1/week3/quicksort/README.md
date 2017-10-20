# Quicksort

[Wikipedia Page](https://en.wikipedia.org/wiki/Quicksort)

Quicksort is a sort algorithm. The key of the algorithm is how to choose the pivot element. The assignment was to write three different versions:

1. always choose the first element
2. choose a random element
3. choose the "median of 3": take the first, last, and middle element of the array, then choose as the pivot the median of those three

This code runs all three versions of the algorithm on the same input (given by `./quicksort.txt`, not included in the repo to help prevent Coursera cheating) and outputs the number of comparisons peformed by the alorithm (which was the coursera assignment)

example:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/quicksort`
input length is: 10000
First: Did xxxxxx comparisons
Last: Did xxxxxx comparisons
MoTh: Did xxxxxx comparisons
...
```
