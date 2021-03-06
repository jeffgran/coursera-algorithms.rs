# Count Inversions

An "inversion" is defined as a pair of elements in the array that occur in non-sorted order.

Amazingly, a fast (`O(n log(n))`) way to calculate the number of inversions is to Merge-Sort the list, and just keep track of a little bookkeeping meanwhile.

So I just copied my [Merge Sort](../../week1/mergesort) agorithm and added the bookkeeping code.

This program expects a file called `ints.txt` to exist in this directory with a list of numbers, one per line.

### example output

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/count_inversions`
Input: [84, 34, 245, 176, 104, 199, 199, 201, 56, 17, 4, 190, 49, 70, 162, 193, 77, 146, 26, 105, 89, 116, 173, 245, 100, 201, 92, 10, 197, 222, 38, 146, 111, 214, 155, 113, 23, 123, 39, 20, 2, 124, 179, 96, 81, 162, 106, 192, 38, 254, 124, 39, 4, 35, 86, 111, 42, 127, 43, 228, 18, 190, 144, 134, 95, 38, 106, 19, 225, 132, 45, 1, 228, 213, 44, 169, 72, 87, 73, 128, 76, 135, 109, 44, 249, 196, 38, 111, 69, 238, 26, 146, 139, 149, 98, 49, 193, 142, 246, 192, 99, 126, 89, 107, 250, 223, 232, 66, 205, 52, 90, 162, 70, 227, 5, 133, 235, 120, 55, 23, 116, 213, 191, 30, 222, 205, 127, 202, 151, 99, 74, 122, 22, 237, 150, 67, 68, 50, 24, 192, 79, 222, 80, 75, 165, 230, 220, 160, 7, 86, 105, 34, 236, 109, 52, 127, 143, 171, 31, 58, 154, 197, 194, 41, 193, 200, 203, 211, 216, 169, 146, 138, 18, 23, 94, 150, 163, 9, 21, 195, 130, 56, 72, 243, 166, 21, 164, 226, 190, 30, 161, 251, 136, 126, 41, 80, 82, 111, 199, 226, 208, 216, 223, 34, 12, 176, 205, 146, 73, 172, 189, 193, 62, 187, 87, 54, 168, 107, 76, 75, 228, 13, 229, 52, 48, 51, 131, 144, 231, 23, 115, 198, 96, 99, 155, 192, 130, 10, 102, 249, 245, 134, 24, 241, 156, 44, 146, 196, 238, 137, 225, 133, 201, 255, 59, 255]
Number of inversions: 14936
```
