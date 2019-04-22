use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("Please enter the input");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fail to read the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number to play the game");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again"),
            Ordering::Greater => println!("Too big! Try again"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
}
