// CMPT383 - Assignment 2
// Author: Adam Tuck - 301232782
// Exercise 5 in Rust

fn fib(n: usize) -> i32 {
    let mut fibs = Vec::new();
    fibs.push(1);
    fibs.push(1);
    if n==0 || n==1 {
        return fibs[n];
    } else {
        for i in 2..n+1 {
            let first =  fibs[i-1];
            let second = fibs[i-2];
            fibs.push(first+second);
        }
        return fibs[n];
    }
}

fn main() {
    for i in 1..=40 {
        println!("fib({}): {:?}", i, fib(i));
    }   
}
