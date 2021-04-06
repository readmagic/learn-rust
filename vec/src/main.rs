fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let x = vec![1, 2, 3];
    let z = match x.get(2) {
        Some(xz) => *xz,
        None => 0,
    };

    println!("{}", z);
    println!("{}", v[1]);
    let mut nnn = vec![1, 2, 3];
    //直接奔溃
    //let nnn1 = &nnn[100];
    //如果越界返回None
    let nnn2 = nnn.get(100);

    //遍历值
    for i in &nnn {
        println!("======>{}", i);
    }

    //遍历并修改值
    for i in &mut nnn {
        *i += 60;
    }

    for i in &nnn {
        println!("======>{}", i);
    }

}
