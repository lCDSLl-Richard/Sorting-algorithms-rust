use std::vec;

pub fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = false;
    for i in 0..arr.len() - 1 {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min)
    }
}

fn merge(arr: &mut [i32], start: usize, mid: usize, end: usize) {
    let l_size = mid - start + 1;
    let r_size = end - mid;

    let mut left_arr = vec![0; l_size];
    let mut right_arr = vec![0; r_size];

    for i in 0..l_size {
        left_arr[i] = arr[i + start];
    }

    for i in 0..r_size {
        right_arr[i] = arr[i + mid + 1];
    }

    let mut l_i: usize = 0;
    let mut r_i: usize = 0;
    let mut m_i = start;

    while l_i < l_size && r_i < r_size {
        if left_arr[l_i] < right_arr[r_i] {
            arr[m_i] = left_arr[l_i];
            l_i += 1;
        } else {
            arr[m_i] = right_arr[r_i];
            r_i += 1;
        }
        m_i += 1;
    }

    while l_i < l_size {
        arr[m_i] = left_arr[l_i];
        l_i += 1;
        m_i += 1;
    }

    while r_i < r_size {
        arr[m_i] = right_arr[r_i];
        r_i += 1;
        m_i += 1;
    }
}

pub fn merge_sort(arr: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mid = start + (end - start) / 2;

    merge_sort(arr, start, mid);
    merge_sort(arr, mid + 1, end);
    merge(arr, start, mid, end);
}
