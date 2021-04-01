#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //方法
    fn area(&self) -> u32 {
        self.height * self.width
    }
    //方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

//可以使用多个impl,和C++一样
impl Rectangle {
    fn say_hi() {
        println!("Hello");
    }
}


fn main() {
    let rect = Rectangle {
        width: 30,
        height: 20,
    };
    let rect2 = Rectangle { width: 10, height: 10 };
    let rect3 = Rectangle { width: 40, height: 30 };
    println!("area= {}", rect.area());
    println!("rect2 can hold rect = {}", rect2.can_hold(&rect));
    println!("rect3 can hold rect = {}", rect3.can_hold(&rect));
    //构造正方形
    let square = Rectangle::square(30);
    println!("square = {:#?}", square);
    Rectangle::say_hi();
}
