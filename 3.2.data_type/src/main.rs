fn main() {
    //基本类型有四种，整型，浮点型，布尔型，字符型
    //整型 默认给的是i32
    //有符号 i8,i16,i32,i64,isize
    //无符号 u8,u16,u32,u64,usize
    let x = 0b111_000;//二进制
    println!("二进制表示 {}", x);
    let x = 0o77;//八进制
    println!("八进制表示 {}", x);
    let x = 0xff;//十六进制
    println!("十六进制表示 {}", x);
    //溢出的情况,在编译期就给你报错
    // let x: i8 = 256;
    // println!(" {}", x);

    //浮点型 f32,f64,默认给的是f64
    let y = 3.14159;
    println!("浮点型 {}", y);
    //布尔型 true,false
    let a = true;
    let b = false;
    println!("布尔型 {},{}", a, b);
    //字符型
    let c = 'z';
    println!("字符型 {}", c);
    //============================
    //复合类型 元组、数组
    let tup = ('c', 3.14, 3);
    println!("元组 {:?}", tup);
    let array = [34, 4, 2, 4, 5];
    for value in array.iter() {
        println!("数组 value={}", value);
    }
    //两个数值交换
    let (a, b) = (10, 20);
    let (b, a) = (a, b);
    println!("a={},b={}", a, b);
}
