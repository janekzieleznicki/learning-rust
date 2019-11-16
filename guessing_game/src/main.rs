use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn print_start() {
    println!("Guess the number!");

    println!("Please input your guess.");
}

fn get_guess() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    guess
}

fn get_randome() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    return secret_number;
}

fn my_main() {
    print_start();
    println!(
        "You guessed {}, but i guessed {}",
        get_guess(),
        get_randome()
    );
}

fn main() {
    // ---snip---
    print_start();
    let guess: i32 = get_guess().trim().parse().expect("Not a number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
