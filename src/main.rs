use std::fs::File;
use std::io::{self, BufRead};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("No arguments passed. Defaulting to sanity checks");
        println!("Usage: s for sanity check, e [number] to read an example file");
        println!("Append n to do naive method");
        sanity_checks();
    }
    else if args[1] == "s" {
        sanity_checks();
    }
    else if args[1] == "e" {
        if args.len() < 3 {
            panic!("Please provide an example number");
        }
        if args.len() >= 4 && args[3] == "n" {
            read_ex(&args[2], true);
        } else {
            read_ex(&args[2], false);
        }
    }
}

fn read_ex(x: &str, naive: bool) {
    // gonna be unsafe because I expect this file to exist
    // and be correctly formatted
    let filename = format!("ex{}.txt", x);
    println!("reading example file number {x}: {filename}");
    let ex = File::open(filename).unwrap();
    let ex_reader = io::BufReader::new(ex).lines();
    let mut ex_vec: Vec<i32> = Vec::new();
    for line in ex_reader.flatten(){
        ex_vec.push(line.parse().unwrap());
    }
    if naive { println!("doing a naive method"); }
    let now = Instant::now();
    let ex_invs = 
        if naive {naive_count(&mut ex_vec)}
        else {mergesort(&mut ex_vec)};
    let elapsed_time = now.elapsed();
    println!("Inversions in example {x}: {ex_invs}");
    println!("Counting those inversions took {:.2?}", elapsed_time);
}

fn sanity_checks() {
    let mut already_sorted = [0, 1, 2, 3, 4, 5];
    let no_inversions = mergesort(&mut already_sorted);
    println!("Inversions in {:?}: {no_inversions}", already_sorted);
    let mut inverse_sorted =  [8, 7, 6, 5, 4, 3, 2, 1];
    let inverse_inversions = mergesort(&mut inverse_sorted);
    println!("Inversions in {:?}: {inverse_inversions}", inverse_sorted);
    let expected_inversions = (inverse_sorted.len() * (inverse_sorted.len() - 1)) / 2;
    println!("(should be {expected_inversions})");
    if no_inversions != 0 || inverse_inversions != expected_inversions {
        panic!("sanity checks failed!")
    }
    println!("sanity checks passed")
}

fn mergesort(arr: &mut [i32]) -> usize {
    if arr.len() < 2 {
        return 0; // already sorted; no inversions
    }
    // find midpoint
    let mid = arr.len() / 2;

    // create duplicate array to overwrite with the results of the sorted array
    let mut sorted = arr.to_vec();

    // recursively sort both halves and get each half's inversions
    let left_inv = mergesort(&mut arr[..mid]);
    let right_inv = mergesort(&mut arr[mid..]);

    // merge both halves into the duplicate array; count split inversions
    let split_inv = merge(&arr[..mid], &arr[mid..], &mut sorted);

    // copy duplicate array back into original array
    // which calling scope will have access to
    arr.copy_from_slice(&sorted);
    
    // total up inversions and return
    left_inv + right_inv + split_inv
}

fn merge(left_arr: &[i32], right_arr: &[i32], sorted: &mut [i32]) -> usize {
    // initialize pointers for both input arrays, the output array,
    // and a count for inversions
    let (mut l_ptr, mut r_ptr, mut s_ptr, mut inversions) = (0, 0, 0, 0);
    // loop until one or the other array is exhausted
    while l_ptr < left_arr.len() && r_ptr < right_arr.len(){
        // if right array's member is greater than left array's member,
        // put right array's member in the final array and
        // increment the number of inversions by the number of remaining members
        // of left array
        if right_arr[r_ptr] < left_arr[l_ptr] {
            sorted[s_ptr] = right_arr[r_ptr];
            inversions += left_arr.len() - l_ptr;
            // since we've processed the member of right array, increment
            // the corresponding pointer
            r_ptr += 1;
        }
        // NB. simplifying assumption: no numbers in the original array are exactly equal
        // if the numbers being merged are equal, nothing will be added to inversions
        // which of course may be incorrect. a more explicit definition of inversion would
        // be required to remove this assumption.
        else {
            sorted[s_ptr] = left_arr[l_ptr];
            // since we've processed the member of left array, increment
            // the corresponding pointer
            l_ptr += 1;
        }
        // increment the pointer for the sorted array
        s_ptr += 1;
    }
    // finish whichever array remains
    while l_ptr < left_arr.len() {
        sorted[s_ptr] = left_arr[l_ptr];
        l_ptr += 1;
        s_ptr += 1;
    } while r_ptr < right_arr.len() {
        sorted[s_ptr] = right_arr[r_ptr];
        r_ptr += 1;
        s_ptr += 1;
    }
    // return number of split inversions
    inversions
}

fn naive_count(arr: &[i32]) -> usize {
    let mut inversions = 0;
    for (i, x) in arr.iter().enumerate() {
        for y in arr[i..].iter() {
            if x > y {
                inversions += 1;
            }
        }
    }
    inversions
}
