use rand::prelude::*;

pub fn fill_random(arr: &mut [i32]) {
    let mut rng = rand::thread_rng();

    for i in 0..arr.len() {
        arr[i] = rng.gen_range(0..101);
    }
}

pub fn display_arr(arr: &[i32]) {
    let max: usize;

    if arr.len() > 10 {
        max = 10;
    } else {
        max = arr.len();
    }

    print!("[ ");
    for i in 0..max {
        print!("{} ", arr[i]);
    }
    println!("]");
}
