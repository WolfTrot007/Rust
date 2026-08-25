// A game of High or Low

//use std::io::{self, Write};
// This use is the same as:
use std::io;
use std::io::Write;

fn main() {
    print!("Welcome to High or Low:\nPlease choose a number between (1-100): ");
    
    let _ = io::stdout().flush();
    let mut guess : String = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    guess.pop(); //This is used as Stdin also grabs the enter key when used
                 // so the pop removes the last used character.
    
    

}
