fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 解构
    let (x, y, _z) = tup;

    println!("x 的值为: {}", x);
    println!("y 的值为: {}", y);

    // 使用索引访问元祖中的值
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);
}