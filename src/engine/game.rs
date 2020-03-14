use rand::Rng;
use std::io;

mod game {
    pub fn play_game() {
        println!("Please guess a number from 1 to 10:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).
            expect("Failed to read a number");
        let guess = rand::thread_rng().gen_range(1, 10);
        println!("Secret number is {}", guess);
    }
}
