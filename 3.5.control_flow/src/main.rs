fn main() {
    //if表达式
    let a = 10;
    if a > 5 {
        println!("分支1");
    } else {
        println!("分支2");
    }
    //没有三元表达式,下面报错
    // let x= a>5?10:20;
    //只能这么写,emmm 和scala一样，作为表达式
    let x = if a > 5 { 10 } else { 20 };
    println!("{}", x);
    //loop 循环
    let mut x = 0;
    loop {
        println!("loop打印了{}次", x);
        x = x + 1;
        if x > 10 {
            break;
        }
    }
    //while循环
    let mut x = 0;
    while x < 10 {
        println!("while打印了{}次", x);
        x = x + 1;
    }
    //for 循环
    let z = [21, 2, 3, 4, 5, 56];
    for i in z.iter() {
        println!("数组的值是 {}", i);
    }
    //这个和python的ranger差不多
    // 1..10 不包含10
    // 1..=10 包含10
    for x in 1..=10 {
        println!("{}", x)
    }
}
