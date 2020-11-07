use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //define datatype of input

    io::stdin() //io = module / stdin = function -> return object
        .read_line(&mut guess) //read input from terminal
        .expect("Failed to read line"); //error handling

    println!("You guessed: {}", guess);
}
