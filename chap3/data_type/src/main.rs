
fn main() {
    tuple();
    arr_test();
}

// 元组
fn tuple() {
    let tup: (i32, f64, u8, String) = (500, 6.4, 1, "James".to_string());
    // 解构
    let (a, b, c, d) = tup;
    println!("a = {}", a);
    println!("b = {}", b);
    println!("c = {}", c);
    println!("d = {}", d);

    // 索引
    let one = tup.2;
    println!("one = {}", one);
}

fn arr_test() {
    // println!("array!");
}
