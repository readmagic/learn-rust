//标准的结构体
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: i16,
    active: bool,
}

//元组结构体
#[derive(Debug)]
struct Point(i32, i32);

//空结构体
#[derive(Debug)]
struct Empty {}

//持有引用的结构体，clion已经报错了
// #[derive(Debug)]
// struct BadUser {
//     username: &str,
//     email: &str,
//     age: i16,
//     active: bool,
// }


fn main() {
    //需要注意的是，一旦实例可变，那么实例中的所有字段都将是可变的
    let mut user = User {
        username: String::from("张三"),
        email: String::from("303734023@qq.com"),
        age: 28,
        active: true,
    };
    println!("username= {}", user.username);
    //打印结构体
    println!("user {:?}", user);
    //打印结构体 美化版
    println!("user {:#?}", user);
    user.username = String::from("李四");
    println!("username= {}", user.username);
    //调用函数返回结构体
    let user1 = build_user(String::from("小明"), String::from("3232@qq.com"), 12);
    println!("user1 = {:#?}", user1);
    //完全复制一个结构体的数据,和JavaScript一样
    let user2 = User {
        ..user1
    };
    println!("user2 = {:#?}", user2);
    //部分复制一个结构体的数据
    let user3 = User {
        username: String::from("小红"),
        email: String::from("5233312@qq.com"),
        ..user1
    };
    println!("user3 = {:#?}", user3);

    //type是这样使用的
    // type Point = (i32, i32);
    // let x: Point = (12, 21);
    // println!("{}", x.0);

    //元组结构体是这样使用的
    let x = Point(12, 21);
    println!("元组结构体的值={:#?}", x);
    let empty = Empty {};
    println!("空结构体的值={:#?}", empty);
    //下面的结构体持有引用，clion已经报错了
    // let badUser = BadUser {
    //     username: "张三",
    //     email: "303734023@qq.com",
    //     age: 28,
    //     active: true,
    // };
    // println!("结构体持有引用的值={:#?}", badUser);
}

//变量名和字段名相同时，可以简写，和JavaScript一样
fn build_user(username: String, email: String, age: i16) -> User {
    User {
        username,
        age,
        email,
        active: true,
    }
}
