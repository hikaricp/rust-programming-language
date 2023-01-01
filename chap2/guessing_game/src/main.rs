use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数字游戏!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("神秘数字是: {}", secret_number);

    loop {
        println!("请输入一个数字");

        // 使用 mut 关键字声明一个变量是可变的
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 允许使用同名变量 shadow
        // u32 无符号整数类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 出现错误时跳过本次循环
            Err(_) => continue,
        };

        println!("你猜测的数字是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                // 猜对时结束循环
                println!("You win!");
                break;
            }
        }
    }
}
