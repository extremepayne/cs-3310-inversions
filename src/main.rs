fn main() {
    println!("Hello, world!");
}

fn mergesort(arr: &mut [i32]) -> usize {
    if arr.len() < 2 {
        return 0; // already sorted, no inversions
    }
    // find midpoint
    let mid = arr.len() / 2;

    // create duplicate array to overwrite with the results of the sorted array
    let mut sorted = arr.to_vec();

    // recursively sort both halves
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

