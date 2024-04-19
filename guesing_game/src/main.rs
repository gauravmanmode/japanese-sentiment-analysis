use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("value must be between 1 & 100");
        }
        Guess{value}
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your number ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
        let guess = Guess::new(guess).value();
        
        print!("a");

re
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You win");
                break;
            }
        }
    }
}