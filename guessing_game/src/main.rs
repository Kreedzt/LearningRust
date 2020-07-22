use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secrect_number = rand::thread_rng().gen_range(1, 101);

    println!("Guess the number!");

    // println!("The secrect number is {}", secrect_number);

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess_num);

        match guess_num.cmp(&secrect_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }    
}
