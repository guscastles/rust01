use rand::Rng;
use std::io;

const END_GAME: u32 = 5;

pub fn play_game() {
    initial_message();
    let secret = secret_number();
    let mut counter: u32 = 0;
    while counter < END_GAME {
        counter = play(secret, counter);
    }
    println!("Secret number is {}", secret);
}

fn initial_message() {
    println!("Please guess a number from 1 to 10:");
}

fn secret_number() -> i32 {
    return rand::thread_rng().gen_range(1, 11);
}

fn play(secret: i32, counter: u32) -> u32 {
    let guess = read_new_number();
    if secret != guess && counter < (END_GAME - 1) {
        println!("Please, try again :)");
        return counter + 1;
    }
    return END_GAME;
}

fn read_new_number() -> i32 {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
        expect("Failed to read a number");
    return match guess.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    };
}
