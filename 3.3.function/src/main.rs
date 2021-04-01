fn main() {
    other();
    other1(1, 2);
    println!("结果是 {}", other2(23, 3));
    println!("这个结果会编译不过 {}", other3(3, 2));
}

fn other() {
    println!("这是另一个函数");
}

//函数名字不能一样
fn other1(x: i32, y: i32) {
    println!("{}，{}", x, y);
}

//和scala很像,定义了函数返回值，返回值是表达式且没有;
fn other2(a: i32, b: i32) -> i32 {
    a + b
}

//Rust默认返回了一个空元组
fn other3(a: i32, b: i32) -> i32 {
    a + b;//多了个分号，返回的是个空元组
}
