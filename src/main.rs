/*extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
*/

extern crate csv;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
/*
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
    */


    let mut rdr = csv::Reader::from_file("./data/mnist_train.csv").unwrap();
    
    for record in rdr.decode() {
        let mut v = vec![0.0; 1000];
        v = record.unwrap();
        //println!("{:?}", v);
    }

    
}
