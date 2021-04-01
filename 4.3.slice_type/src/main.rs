fn main() {
    let s = String::from("Hello world");
    //切片 和python中差不多
    let z = &s[0..5];
    println!("{}", z);
    println!("{}",first_word(&s));
    //字符串切片的边界必须位于有效的UTF-8字符边界内。
    //let a = "你好世界";
    //let b = &a[0..5];
    //println!("{}",b);
    //数组切片
    let a= [2,3,4,5,6];
    let b:&[i32] = &a[1..=3];
    println!("{}",b.len());
}

fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i]
        }
    }
    &s[..]
}

//
// fn first_word(str: &String) -> &str {
//     str.split_whitespace().collect::<Vec<&str>>()[0]
// }

