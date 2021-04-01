fn main() {
    for i in 1..=10{
        println!("结果 {}", fib(i));
    }

}

fn fib(num: u32) -> u32 {
    if num == 2 || num == 3 {
        1
    } else if num == 1 {
        0
    } else {
        fib(num - 1) + fib(num - 2)
    }
}
