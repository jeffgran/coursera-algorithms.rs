# Closest Pair

This algorithm finds the closest pair of points on a 2D plane.

Two algorithms are implemented. 

1. The naive algorithm is O(n²)
2. The fast algorithm is O(n log n)


There are some comparisons built in to show the run-time difference.


### example output

```
$ cargo run -- 10000
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/closestpair 10000`
-------------------------------------------------------
Slow algorithm:
Closest pair of points was: Point { x: -8141, y: -3422 }, Point { x: -8143, y: -3423 }
Distance was: 2.23606797749979
-------------------------------------------------------
Fast algorithm:
Closest pair of points was: Point { x: -8143, y: -3423 }, Point { x: -8141, y: -3422 }
Distance was: 2.23606797749979
-------------------------------------------------------
```


### benchmark

example output with n = 10000 (i.e. 10,000 points on the graph)

```
$ cargo bench
    Finished release [optimized] target(s) in 0.0 secs
     Running target/release/deps/closestpair-1c11073ab86140bc

running 5 tests
test tests::test_case_1 ... ignored
test tests::test_case_2 ... ignored
test tests::test_delta ... ignored
test tests::bench_n2    ... bench: 362,172,768 ns/iter (+/- 57,898,051)
test tests::bench_nlogn ... bench:   3,902,067 ns/iter (+/- 228,327)
```


Looks like in this case the nlogn algorithm is roughly two orders of magnitude faster than the naive implementation. And it just grows faster as the input gets larger! This was really cool to see after spending all weekend trying to understand and get this crazy algorithm working. :)
