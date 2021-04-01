fn main() {

    //可变的变量
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
    //不可变的变量
    let y = 6;
    println!("不可变的变量 {}", y);
    //以下不可变，赋值会报错
    // y=7;
    //常量的声明必须指定明确的数据类型
    const MAX_POINT: i32 = 100_00;
    println!("常量 {}", MAX_POINT);

    //隐藏，可以不需要额外声明变量，重复利用同一个变量
    let spaces = "    ";//此时，变量spaces是字符串类型
    let spaces = spaces.len();//此时 变量spaces变成了数字类型,原来的spaces被隐藏
    println!("spaces的值是{}", spaces);
    //上面这段代码和下面这段不同,下面这段报错，因为spaces是可变的,可变的必须数据类型一致
    // let mut spaces ="    ";
    // spaces = spaces.len();
}
