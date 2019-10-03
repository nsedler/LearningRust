extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

#[allow(non_snake_case)]
fn main() {

    let randomNum = rand::thread_rng().gen_range(0, 101);

    loop {

        println!("Guess the correct number\n>>>");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("expected a number");

        let guess2: i32 = guess.trim().parse()
            .expect("gimmi a number");

        match guess2.cmp(&randomNum) {

            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}

