fn main() {
    //option 和scala里面差不多
    let x = Some(1);
    match x {
        Some(z) => println!("{}", z),
        None => println!("None"),
    }

    let z = Some(100);
    match z {
        Some(x) => println!("一种啰嗦的写法:{}", x),
        _ => (),
    }
    //if let语法 简化的写法
    if let Some(x) = z {
        println!("if let 语法,(简化写法):{}", x);
    }
    //None类型主动赋值的话,必须显式声明类型
    let a: Option<i32> = None;
    if let Some(x) = a {
        println!("{}", x);
    } else {
        println!("调用了这里");
    }
}
