// CMPT383 - Assignment 2
// Author: Adam Tuck - 301232782
// Exercise 2 in Rust

fn divisors(n: i32) -> Vec<i32> {
    let mut divs = Vec::new();

    // 2->n/2 because no divisor can
    // be bigger than n/2
    for i in 2..=(n/2) {
        if n%i == 0 { 
            divs.push(i); 
        }
    }

    return divs;
}

fn primes(n: i32) -> Vec<i32> {
    let mut primes = Vec::new();
    
    // if number has no divisors it is prime
    for i in 2..=n {
        if divisors(i).is_empty() {
            primes.push(i);
        }
    }

    return primes;
}

fn pythagorean(n: i32) -> Vec<(i32,i32,i32)> {
    let mut abc = Vec::new();

    for a in 1..=n {
        for b in a..=n {
            for c in a..=n {
                if c*c == a*a + b*b {
                    abc.push((a,b,c));
                }
            }
        }
    }

    return abc; 
}

// Wrote before discovering {:?} syntax
fn print_i32_vec(v: Vec<i32>) {
    print!("[");
    for (i, val) in v.iter().enumerate() {
        if i+1==v.len() { 
            print!("{}", val); 
            break; 
        }
        print!("{}, ", val);
    }
    print!("]");
    println!();
}

// Wrote before discovering {:?} syntax
fn print_tup_vec(v: Vec<(i32,i32,i32)>) {
    print!("[");
    for (i, val) in v.iter().enumerate() {
        if i+1==v.len() { 
            print!("({},{},{})", val.0,val.1,val.2); 
            break; 
        }
        print!("({},{},{}), ", val.0,val.1,val.2);
    }
    print!("]");
    println!();
}


fn main() {
    println!("divisors(30): {:?}", divisors(30)); 
    println!("divisors(63): {:?}", divisors(63));
    println!("divisors(127): {:?}", divisors(127));
    println!("primes(7): {:?}", primes(7));
    println!("primes(100): {:?}", primes(100));
    println!("pythagorean(10): {:?}", pythagorean(10));
    print!("pythagorean(30): {:?}", pythagorean(30));
}
