fn main() {
    print();
    dbg();
}

// `derive` 属性自动创建实现
// 需要使用 `fmt::Debug` 使这个 `struct` 可打印
#[derive(Debug)]
#[allow(dead_code)] // 消除代码未被使用的警告信息
struct User {
    name: String,
    age: i32,
    email: Option<String>, // 可选参数，如果没有则为 None
}

fn print() {
    println!("\n*********** 无参数 ***********\n");
    // output: Hello, world!
    println!("Hello, world!");

    println!("\n*********** 多个参数 ***********\n");
    // output: hello rust
    println!("{} {}", "hello", "rust");

    println!("\n*********** 参数重排序 ***********\n");
    // 使用参数索引重新组合排序，支持键值对
    // ⚠️ 注意：属性参数位置必须在索引参数之后，否则会报错：positional arguments cannot follow named arguments
    // output: "lencx": 1 2 rust 3
    println!("{name:?}: {0:?} {1:?} {lang} {2:?} ",
        1, // 1:?
        2, // 2:?
        3, // 3:?
        lang="rust", // lang - 字符串格式输出，不带双引号
        name="lencx", // name:? - 字符串格式输出，带双引号
    );

    println!("\n*********** Struct ***********\n");
    // output: User { name: "lencx", age: 18, email: None }
    let user = User { name: "lencx".into(), age: 18, email: None };
    println!("{:?}", user);

    println!("\n----- Struct 格式美化 -----\n");
    // output:
    // User {
    //  name: "lencx",
    //  age: 18,
    //  email: None,
    // }
    println!("{:#?}", user);

    println!("\n*********** 进制转换 ***********\n");
    println!("进制 10:               {}",   69420); // 69420
    println!("进制 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("进制 8 (octal):        {:o}", 69420); // 207454
    println!("进制 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("进制 16 (hexadecimal): {:X}", 69420); // 10F2C

    println!("\n*********** 补齐 ***********\n");
    println!("\n----- 补空字符串 -----\n");
    // 右对齐：字符左边补充 15 个空字符
    // output: "          hello"
    println!("{0:>15}", "hello");
    // 右对齐：字符左边补充 10 个空字符
    // output: "Name:     lencx"
    println!("Name:{name:>10}", name="lencx");

    println!("\n----- 数字补零 -----\n");
    // 用额外的零填充在数字后面
    println!("{number:0<5}", number=1); // 10000
    // 用额外的零填充在数字前面
    println!("{number:0>5}", number=1); // 00001
    // width: 通过附加“$”在格式说明符中使用命名参数，需要在数字前补零的个数
    println!("{number:0>width$}", number=5, width=10); // 0000000005

    println!("\n----- 周围变量捕获参数 -----\n");
    // 对于 Rust 1.58 及更高版本，您可以直接从周围变量中捕获参数，a 和 b 为变量
    let a: f64 = 1.0;
    let b: usize = 5;
    println!("{a:0>b$}"); // 补零：00001
    println!("{a:>b$}");  // 补空: "    1"
}

fn dbg() {
    // [src/main.rs:90] user = User {
    //     name: "lencx",
    //     age: 18,
    //     email: None,
    // }
    let user = User { name: "lencx".into(), age: 18, email: None };
    dbg!(user);

    // [src/main.rs:95] "a" = "a"
    // [src/main.rs:95] "b" = "b"
    // [src/main.rs:95] 1 = 1
    dbg!("a", "b", 1);
}