///通常见到的三种注释
/// 行注释 //
/// 文档注释 ///
/// 模块注释 //!
/// 兼容C的注释 /**/ 尽可能不要用
fn main() {
    //行注释
    // 打印hello world
    println!("Hello, world!");
}

/// 文档注释 /// ,一般用于函数或结构体（字段）的说明，置于要说明的对象上方。文档注释内部可使用markdown格式的标记语法，可用于 rustdoc 工具的自动文档提取
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}


/// 模块注释使用 //!，用于说明本模块的功能。一般置于模块文件的头部。

/// Rust 也支持兼容 C 的块注释写法：/* */。但是不推荐使用，请尽量不要使用这种注释风格（会被鄙视的）。
