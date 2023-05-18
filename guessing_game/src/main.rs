use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let min = 1;
    let max = 100;
    let secret_number = rand::thread_rng().gen_range(min..=max);
    loop {
        
        println!("Please enter a num between {min} and {max}: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let  guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed a smaller number"),
            Ordering::Greater => println!("You guessed a bigger number!"),
            Ordering::Equal => {
                println!("You guessed the right number!");
                break;
            }
        }
    }

}
