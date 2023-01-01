fn main() {
    // 整数
    // 占用1个字节
    let i8_number: i8 = -100; // 8位的有符号整数
    let u8_number: u8 = 100; // 8位的无符号整数

    println!("i8_number is {}", i8_number);
    println!("u8_number is {}", u8_number);

    // 占用2个字节
    let i16_number: i16 = -200; // 16位的有符号整数
    let u16_number: u16 = 200; // 16位的无符号整数

    println!("i16_number is {}", i16_number);
    println!("u16_number is {}", u16_number);

    // 占用4个字节
    let i32_number: i32 = -300; // 32位的有符号整数
    let u32_number: u32 = 300; // 32位的无符号整数

    println!("i32_number is {}", i32_number);
    println!("u32_number is {}", u32_number);

    // 占用8个字节
    let i64_number: i64 = -400; // 64位的有符号整数
    let u64_number: u64 = 400; // 64位的无符号整数

    println!("i64_number is {}", i64_number);
    println!("u64_number is {}", u64_number);

    // arch 根据系统架构决定
    let isize_number: isize = 100;
    let usize_number: usize = 100;

    // 浮点数
    let x = 2.0; // f64 默认浮点类型, 双精度
    let y: f32 = 3.0; // f32, 单精度

    // 字符
    let c = 'z';
    let z = 'Z';
    let smile = '😄';

    // 布尔
    let t = true;
    let f: bool = false; // 显示类型标注

    // 数值运算
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;
}