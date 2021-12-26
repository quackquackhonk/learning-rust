use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // generate a secret number using the rand crate
    let secret_number = rand::thread_rng().gen_range(1..101); 

    println!("Guess the number!");

    loop {

        println!("Please input your guess.");

        // create a mutable string for input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)          // read a line from standard in
            .expect("Failed to read line"); // handle io exceptions

        // SHADOWS guess with a NEW unsigned 32 bit int
        //  vv new              vv old (String)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess); // print the guess

        // check if the guess is correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
