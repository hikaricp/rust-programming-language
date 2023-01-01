fn main() {
    println!("数组");

    // 指定数组元素类型及元素个数
    let arr:[i32; 5] = [1, 2, 3, 4, 5];
    println!("num is {}", arr[0]);

    let names = ["James", "KB", "周杰伦"];
    let first = names[0];
    let second = names[1];
    println!("0 {}", first);
    println!("1 {}", second);

    let a = [1, 2, 3, 4, 5  ];
    let index = 10;
    // 数组越界
    let element = a[index];
    println!("The value is {}", element);
}