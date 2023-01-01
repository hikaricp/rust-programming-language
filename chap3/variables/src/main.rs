// 声明常量，必须指定类型
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x is {}", x);

    // Shadowing
    let foo = 100;
    let foo = foo + 1;
    let foo = foo * 2;
    println!("foo is {}", foo);

    // Shadowing 和 mut
    // Shadowing 同名的变量类型可以不一致
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces is {}", spaces);

    // mut 变量类型必须一致
    // let mut spaces = "    ";
    // 错误的写法
    // spaces = spaces.len();
}
