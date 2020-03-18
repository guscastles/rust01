//! Guessing Game
//! Experimental code in Rust
//! The player has 5 chances to guess a random number.

mod engine;
use newmath;

fn main() {
    println!("This is the function from the newmath module: {}",
             newmath::addition_plus_one(1, 2));
    engine::play()
}
