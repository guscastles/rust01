/// Messages module for the game engine
///
/// Provides message functions for convenience.


pub const READ_FAILURE: &str = "Failed to read a number";

pub fn initial_message() {
    println!("Please guess a number from 1 to 10:");
}

pub fn final_message(secret: u32) {
    println!("Secret number is {}", secret);
}

pub fn try_again() {
    println!("Please, try again :)");
}
