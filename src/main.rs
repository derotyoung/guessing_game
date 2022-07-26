use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    // println!("You guessed: {}", guess);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess_num);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    fn value(&self) -> u32 {
        self.value
    }
}
