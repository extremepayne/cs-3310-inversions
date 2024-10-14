# cs-3310-inversions

Program that counts the inversions (pairs of out-of-order elements) of an array of integers. Made for UVU CS 3310 Analysis of Algorithms. Designed as a solution to Tim Roughgarden's Algorithms Illuminated problem 3.5.

Includes both O(n²) naive version using nested for loops and O(n\*log(n)) version using mergesort.

This assignment helped me get comfortable actually using Rust slices to pass array subsegments back and forth through the recursion.

## Example usage:

`cargo run --release` or `cargo run --release -- s`

Perform "sanity checks" -- basic example problems with known/obvious solutions we can check against to ensure the algorithm is (probably) working.

`cargo run --release -- e [num]`

Read in an example dataset from a file. Files are named `ex[num].txt`. Counts inversions using mergesort; prints output and the time it takes to compute.

`cargo run --release -- e [num] n`

Reads in an example dataset as above. Counts inversions using the naive nested for loops method; prints output and time it takes to compute.

Example datasets taken from [algorithmsilluminated.org](https://algorithmsilluminated.org).

## Example results

Here's some results from running on my machine.

```
$ cargo run --release -- e 2

reading example file number 2: ex2.txt
Inversions in example 2: 2407905288
Counting those inversions took 25.64ms

$ cargo run -- release -- e 2 n

reading example file number 2: ex2.txt
doing a naive method
Inversions in example 2: 2407905288
Counting those inversions took 948.10ms
```

## Conclusions from this assignment

I noticed, as others did, that on small datasets (such as `ex1/txt`) the divide-and-conquer algorithm is not guaranteed to be faster; it can sometimes be slower. However, on very large datasets (like `ex2.txt`) the divide-and-conquer algorithm is always faster. This is the basic principle of asymptotic behavior that underlies big-O notation.

However, unlike my friends who did this assignment in Python, my naive method finishes in a reasonable amount of time. Theirs took 3 minutes to go over `ex2.txt` while mine finishes in about a second. This demonstrates the advantage of using a fast language, even if it takes a bit to wrap your head around how to program in that language.
