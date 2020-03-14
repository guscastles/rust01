use rand::Rng;
use std::io;
use std::convert::TryInto;

mod messages;

const END_GAME: u32 = 5;
const NOT_IN_SCOPE: u32 = 0;

pub fn play() {
    messages::initial_message();
    let secret = secret_number();
    run(secret);
    messages::final_message(secret);

}

fn secret_number() -> u32 {
    return rand::thread_rng().gen_range(1, 11);
}

fn run(secret: u32) {
    let mut counter: u32 = 0;
    while counter < END_GAME {
        counter = check_guess(secret, counter);
    }
} 

fn check_guess(secret: u32, counter: u32) -> u32 {
    let guess = read_new_number();
    if secret != guess && counter < (END_GAME - 1) {
        messages::try_again();
        return counter + 1;
    }
    return END_GAME;
}

fn read_new_number() -> u32 {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).
        expect(messages::READ_FAILURE);
    return match guess.trim().parse::<i32>() {
        Ok(num) => num.try_into().unwrap(),
        Err(_) => NOT_IN_SCOPE,
    };
}
