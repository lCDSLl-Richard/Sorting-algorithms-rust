mod sorting;
mod utils;

use sorting::{bubble_sort, merge_sort, selection_sort};
use std::time::Instant;
use utils::{display_arr, fill_random};

const SIZE: usize = 10000;

fn main() {
    let mut arr: [i32; SIZE] = [0; SIZE];
    fill_random(&mut arr);

    println!("Bubble sort");
    println!("Before sorting: ");
    display_arr(&arr);

    let mut time = Instant::now();
    bubble_sort(&mut arr);
    let mut duration = time.elapsed().as_micros();

    println!("After sorting: ");
    display_arr(&arr);
    println!("Done in: {}ns", duration);

    fill_random(&mut arr);

    println!("\nMerge sort");
    println!("Before sorting: ");
    display_arr(&arr);

    time = Instant::now();
    let end = arr.len() - 1;
    merge_sort(&mut arr, 0, end);
    duration = time.elapsed().as_micros();

    println!("After sorting: ");
    display_arr(&arr);
    println!("Done in: {}ns", duration);

    fill_random(&mut arr);

    println!("\nSelection sort");
    println!("Before sorting: ");
    display_arr(&arr);

    time = Instant::now();
    selection_sort(&mut arr);
    duration = time.elapsed().as_micros();

    println!("After sorting: ");
    display_arr(&arr);
    println!("Done in: {}ns", duration);
}
