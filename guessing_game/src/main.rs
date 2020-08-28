use std::io;
use rand::Rng; // Rng 'trait' defines methods that random number generators implement
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Guess the number!");

    // equivalent to while { true }
    loop {
        println!("Please input your guess:");

        // new() is an associated function (static method) of String
        let mut guess = String::new();

        // stdin is a function, but it returns a Stdin object
        // '&' means the argument is a reference (which is also immutable by default, hence the '&mut')
        io::stdin()
            .read_line(&mut guess) // returns an io::Result() object
            .expect("Failed to read line");

        // shadow previous guess variable.
        let guess: u32 = guess.trim().parse().expect("Please type a number.");

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed the number, you win!");
                break;
            }

        }
    }

    println!("The secret number is {}!", secret_number);
}
