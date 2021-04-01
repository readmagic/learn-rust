fn takes_ownership(x: String) {
    println!("{}", x);
}

fn gives_ownership(x: String) -> String {
    println!("{}", x);
    x
}


fn make_copy(x: i32) {
    println!("{}", x);
}


//所有权
fn main() {
    //字符串字面量，是不可变的
    let x = "test";
    //字符串变量
    let mut y = String::from("hello");
    y.push_str(",world");
    println!("{},{}", x, y);
    //移动（浅拷贝）
    let s1 = String::from("hei");
    //let s2 = s1;
    //println!("{},{}", s1, s2);//会编译报错，因为内存空间上s1和s2指向了同一块内存地址，s2=s1之后,s1就发生了移动
    //深拷贝,s1和s3不光是指针复制，指向的内容也复制了，两者独立
    let s3 = s1.clone();
    println!("{},{}", s1, s3);
    //数组是不是拥有Copy的trait呢，测试结果是和元组类型，如果数组中包含简单类型(栈上的)就可以，如果是堆上的类型就不行
    //以下可以
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{},{}", arr1[0], arr2[0]);
    //以下不行
    // let z = String::from("hehe");
    // let arr3 = [z];
    // let arr4 = arr3;
    // println!("{},{}", arr3[0], arr4[0]);
    //所有权和函数
    let a = String::from("ce");
    takes_ownership(a);
    // println!("{}",a);// 编译报错,因为a已经被销毁
    let b = 3;
    make_copy(b);
    println!("{}", b);//这个不会报错，因为是在栈上的
    //下面这样就可以了，因为函数运行完了之后将数据返回了
    let a = String::from("ce");
    let a = gives_ownership(a);
    println!("{}",a);
}
