// CMPT383 - Assignment 2
// Author: Adam Tuck - 301232782
// Exercise 4 in Rust

fn hailstone(n: i32) -> i32 {
    return if n%2 == 0 { n/2 } else { 3*n+1 }
}

fn hail_seq(mut n: i32) -> Vec<i32> {
    let mut h = Vec::new();

    h.push(n);
    while n!=1 {
        n=hailstone(n);
        h.push(n);
    } 

    return h; 
}

fn merge(a: &mut Vec<i32>, beg: usize, mid: usize, end: usize, b: &mut Vec<i32>){
    let mut i = beg;
    let mut j = mid;

    for k in beg..end {
        if i < mid && (j >= end || a[i] <= a[j]) {
            b[k] = a[i];
            i += 1;
        } else {
            b[k] = a[j];
            j += 1;
        }
    }
}

fn split_merge(b: &mut Vec<i32>, beg: usize, end: usize, a: &mut Vec<i32>){
    if end - beg < 2 {
        return;
    }
    let mid = (end + beg) / 2;
    split_merge(a, beg, mid, b);
    split_merge(a, mid, end, b);
    merge(b, beg, mid, end, a);
}

fn copy_vector(a: &mut Vec<i32>, beg: usize, end: usize, b: &mut Vec<i32>){
    for i in beg..end{
        b[i] = a[i];
    }
}

// Takes in mutable reference to vector owned in main scope
// and all helper functions pass around mutable references
fn merge_sort(a: &mut Vec<i32>) {
    let n = a.len();
    let mut b = vec![0; n];
    copy_vector(a, 0, n, &mut b);
    split_merge(&mut b, 0, n, a);
}

fn main() {
    println!("hail_seq(1): {:?}", hail_seq(1));
    println!("hail_seq(11): {:?}", hail_seq(11));
    println!("hail_seq(6): {:?}", hail_seq(6));
    // v1 -> v4 get altered so no reference to 
    // original unsorted vector
    let mut v1 = vec![];
    let mut v2 = vec![4];
    let mut v3 = vec![6,2,4,8,9,5,3,1,7,10];
    let mut v4 = vec![1,9,3,2,7,6,4,8,5];
    merge_sort(&mut v1);
    merge_sort(&mut v2);
    merge_sort(&mut v3);
    merge_sort(&mut v4);
    println!("merge_sort([]): {:?}", v1);
    println!("merge_sort([4]): {:?}", v2);
    println!("merge_sort([6,2,4,8,9,5,3,1,7,10]): {:?}", v3);
    println!("merge_sort([1,9,3,2,7,6,4,8,5]): {:?}", v4);
}
