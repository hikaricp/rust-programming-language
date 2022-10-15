use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is : {}", secret_number);

    println!("Please input you guess.");

    // 使用 mut 关键字声明一个变量是可变的
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
