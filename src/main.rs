/// Guessing Game
/// Experimental code in Rust
/// The player has 5 chances to guess a
/// random number

use rand::Rng;

fn main() {
    play_game()
}

fn play_game() {
    println!("Please guess a number from 1 to 10:");
    let guess = rand::thread_rng().gen_range(1, 10);
    println!("Secret number is {}", guess)
}
