use std::io;

fn main() {
    println!("Guess your number");
    println!("please input your number");
    
    let mut guess = String::new();
    


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("You guessed {}", guess);
}