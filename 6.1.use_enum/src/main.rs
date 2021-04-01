//rust不推荐这么写
// #[derive(Debug)]
// enum IpAddKind {
//     V4,
//     V6,
// }
//
// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddKind,
//     address: String,
// }

//Rust推荐这么写,和java的枚举差不多,可以带变量,不一样的是每个枚举值的类型可以不一样
// enum ipAddr{
//     V4(String),
//     V6(String),
// }

enum ipAddr {
    V4(u8, u8, u8, u8),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Change(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("{},{}", x, y),
            Message::Change(x, y, z) => println!("{},{},{}", x, y, z),
            // Write(String),这个使用的是引用,所有会有问题,看看后面章节会不会提到如何处理,现在用_解决
            _ => println!("Other"),
        }
    }
}

fn main() {
    //Rust不推荐的写法
    // let v4 = IpAddKind::V4;
    // let v6 = IpAddKind::V6;
    // println!("{:?}", v4);
    // let ip = IpAddr{
    //     kind:IpAddKind::V4,
    //     address:String::from("127.0.0.1")
    // };
    // println!("{:?}", ip);
    //Rust推荐的写法
    //let v4 = ipAddr::V4(String.from("127.0.0.1"));
    //let v6 = ipAddr::V6(String.from("::1"));

    //不同的枚举值可以包含不同的类型
    //let v4 = ipAddr::V4(127,0,0,1);

    let m = Message::Write(String::from("hei"));
    m.call();
    let n = Message::Change(1, 2, 3);
    n.call();
}
