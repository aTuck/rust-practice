// CMPT383 - Assignment 2
// Author: Adam Tuck - 301232782
// Example 2 in Rust: Digit guessing game

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I've generated a random number and each digit");
    println!("can be from 1-5! Guess the number!");

    let s1 = rand::thread_rng().gen_range(1,5);
    let s2 = rand::thread_rng().gen_range(1,5);
    let s3 = rand::thread_rng().gen_range(1,5);
    let s4 = rand::thread_rng().gen_range(1,5);

    let secret = vec![s1,s2,s3,s4];
    let mut num_correct = 0;

    println!("Please input your guess like \"XXXX\" where");
    println!("X can be any digit 0-9");
    loop {
        num_correct = 0;
        let mut guess_num = 0;
        let mut guess_vec = vec![0,0,0,0];

        // For each digit
        for i in 0..4{
            // Input loop for if they don't input number
            loop {
                let mut input = String::new();
                println!("Guess digit {}!", i+1);
                
                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");

                let guess = input.trim();
                match guess.parse::<u32>() {
                    Ok(num) => {
                        guess_num=num;
                        break;
                    }
                    Err(..) => println!("Please guess an integer"),
                };
            }
            println!("You guessed: {}", guess_num);
            guess_vec[i] = guess_num;
        }
        // Print guess for user to remember
        print!("Your guess: ");
        for i in 0..4 { 
            print!("{}", guess_vec[i]);
        }
        println!();

        // Check correctness
        for i in 0..4 { 
            if guess_vec[i] < secret[i] {
                println!("Digit {} too low!", i);
            } else if guess_vec[i] > secret[i] {
                println!("Digit {} too big!", i);
            } else {
                println!("Digit {} correct!", i);
                num_correct += 1;
            }
        }

        // Win condition
        if num_correct == 4{
            println!("You guessed the number! You win!");
            break;
        }
        println!("Guess again!");
    }
}
