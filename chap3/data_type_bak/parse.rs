fn main() {
    // 标注具体的数据类型
    let guess: u32 = "41".parse().expect("Not a number.");
    println!("value = {}", guess);
}