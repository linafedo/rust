use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let attempt_count = 3;
    let mut step = 1;
    while step <= attempt_count {
        if step <= attempt_count {
            println!("Please, inter your guess");
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(result) => result,
                Err(_) => continue
            };

            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!");
                }
                Ordering::Equal => {
                    println!("You win!");
                    return;
                }
                Ordering::Greater => {
                    println!("Too big!")
                }
            }
        }
        step += 1;
    }
    println!("YOU LOST :(")
}
