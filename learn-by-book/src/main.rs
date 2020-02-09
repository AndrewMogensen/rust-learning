use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the Number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1,101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Input your guess:");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {  // or let guess = guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input numbers only.");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
