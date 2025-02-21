/// 字符串
pub fn string_def() {
    // 什么是字符串
    // Rust 的核心语言中只有一种字符串类型：字符串 slice str，它通常以被借用的形式出现，&str。
    // 字符串 slices：它们是一些对储存在别处的 UTF-8 编码字符串数据的引用。
    //      举例来说，由于字符串字面值被储存在程序的二进制输出中，因此字符串字面值也是字符串 slices。
    // String 和 字符串 slices 都是 UTF-8 编码

    let mut str = String::new();

    // 字符串字面值
    let data = "hello world";

    let s = data.to_string();
    println!("{}", s);

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();
    println!("{}", s);

    let s = String::from("initial contents");
    println!("{}", s);

    // 更新字符串
    // String 的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变 Vec 的内容一样。
    // 另外，可以方便的使用 + 运算符或 format! 宏来拼接 String 值。

    let mut s1 = String::from("foo");
    let s2 = "bar";
    // push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权。
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    // push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中。
    s.push('l');
    println!("s is {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 在相加后不再有效的原因，和使用 s2 的引用的原因，与使用 + 运算符时调用的函数签名有关。
    // + 运算符使用了 add 函数，这个函数签名看起来像这样：
    // fn add(self, s: &str) -> String {
    // 在 add 调用中使用 &s2 是因为 &String 可以被 强转（coerced）成 &str。
    // 当add函数被调用时，Rust 使用了一个被称为 Deref 强制转换（deref coercion）的技术，
    // 你可以将其理解为它把 &s2 变成了 &s2[..]。
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! 与 println! 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 String。
    // 这个版本就好理解的多，宏 format! 生成的代码使用引用所以不会获取任何参数的所有权。
    let s = format!("{s1}-{s2}-{s3}");

    let s1 = String::from("hello");
    // string indices are ranges of `usize`
    // Rust 的字符串不支持索引。
    // let h = s1[0];

    // String 是一个 Vec<u8> 的封装。
    let hello = "Здравствуйте";
    let answer = &hello.as_bytes()[0];
    println!("{}", answer);

    // 从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：字节、标量值和字形簇（最接近人们眼中 字母 的概念）。
    // 请记住有效的 Unicode 标量值可能会由不止一个字节组成。
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

}