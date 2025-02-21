
/// 所有权
pub fn ownership() {
    // Rust 中的每一个值都有一个 所有者（owner）。
    // 值在任一时刻有且只有一个所有者。
    // 当所有者（变量）离开作用域，这个值将被丢弃。

    // 冒号 :: 是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。

    // String类型
    // 被分配到堆上的数据，所以能够存储在编译时未知大小的文本。
    let mut s1 = String::from("hello");
    s1.push_str(" world!");
    println!("{}", s1);

    // 内存与分配
    // 必须在运行时向内存分配器（memory allocator）请求内存。（当调用 String::from 时，它的实现 (implementation) 请求其所需的内存。）
    // 需要一个当我们处理完 String 时将内存返回给分配器的方法。（Rust 采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。）
    // 当 s2 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。
    // 这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。
    {
        let s2 = String::from("hello");
    } // s2 在这里离开作用域，它的值被丢弃，内存被释放。

    // 移动
    // 整型
    // 因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
    // 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效。

    // Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。
    // 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    // 如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。
    // 要学习如何为你的类型添加 Copy 注解以实现该 trait，请阅读附录 C 中的 “可派生的 trait”。
    // 作为一个通用的规则，任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。
    // 如下是一些 Copy 的类型：
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // String
    // String 由三部分组成，一个指向存放字符串内容内存的指针，一个长度，和一个容量。
    // 这一组数据存储在栈上。右侧则是堆上存放内容的内存部分。
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 value borrowed here after move
    // println!("s1 = {}, s2 = {}", s1, s2);
    // 移动操作相对于复制栈上的值，但不复制堆上的内容。
    // 当移动对象是已知数据大小的数据时，复制其值放入栈中
    // 而移动对象是存放在堆上的数据的时候，复制的其实是指针，多个指针数据指向同一个堆空间，所以移动操作不会复制堆上的数据。所以避免了二次释放的问题。
    // Rust 永远也不会自动创建数据的 “深拷贝”。

    // 克隆
    // 复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}


/// 所有权与函数
pub fn ownership_function() {
    // 将值传递给函数与给变量赋值的原理相似。向函数传递值可能会移动或者复制，就像赋值语句一样。

    let s = String::from("hello");

    takes_ownership(s);
    // s 的值已被移动到函数中，所以这里将无法再使用它。
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

/// 返回值与函数
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

/*
    所有权与返回值
*/
fn ownership_return() {
    let s1 = gives_ownership();
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    // s2 的值已被移动到函数中，所以这里将无法再使用它。
    // println!("{} {}", s1, s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/// 引用
pub fn ownership_reference() {
    // 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。
    // 与指针不同，引用确保指向某个特定类型的有效值。
    // 与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。
    let s1 = String::from("hello");
    // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
    // 创建一个引用的行为称为 借用（borrowing）
    // 正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

// 函数签名使用 & 来表明参数 s 的类型是一个引用。
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// 可变引用
pub fn ownership_mutable_reference() {
    let mut s = String::from("hello");
    println!("{}", s);

    change(&mut s);
    println!("{}", s);

    // 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用
    // 防止同一时间对同一数据存在多个可变引用。新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争。
    // 数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
    // 两个或更多指针同时访问同一数据。
    // 至少有一个指针被用来写入数据。
    // 没有同步数据访问的机制。
    // let r1 = &mut s;
    // second mutable borrow occurs here
    // let r2 = &mut s;

    // 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
    // {
    //     let r2 = &mut s;
    // }

    // 我们 也 不能在拥有不可变引用的同时拥有可变引用。
    // 不可变引用的借用者可不希望在借用时值会突然发生改变！然而，多个不可变引用是可以的，因为没有哪个只能读取数据的引用者能够影响其他引用者读取到的数据。
    let r3 = &s;
    let r4 = &s;
    // mutable borrow occurs here
    // let r5 = &mut s;
    // println!("{} {} {}", r3, r4, r5);

    // 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    // 例如，因为最后一次使用不可变引用（println!)，发生在声明可变引用之前，所以如下代码是可以编译的
    println!("{} {}", r3, r4);
    let r5 = &mut s;
    println!("{}", r5);
}

fn change(s: &mut String) {
    s.push_str(", world")
}


/// 悬垂指针
pub fn ownership_dangling_reference() {
    // 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），
    // 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    // 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：
    // 当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

    // 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 引用必须总是有效的。
    let reference_to_nothing = dangle();
}

// expected named lifetime parameter
// fn dangle() -> &String { // dangle 返回一个字符串的引用
fn dangle() -> String { // dangle 返回一个字符串
    let s = String::from("hello"); // s 是一个新字符串
    // &s // 返回字符串 s 的引用
    s // 返回字符串 s
} // 这里 s 离开作用域并被丢弃。其内存被释放。
// 危险！


/// 切片
pub fn owner_slice() {
    let mut s = String::from("hello world");
    // 前闭后开
    let hello = &s[0..5];
    let world = &s[6..11];

    // 如果切片范围是0开头可以省略，如果切片范围是字符串结尾可以省略，如果获取整个字符串可以省略前后范围
    let hello = &s[..5];
    let world = &s[6..];
    let whole = &s[..];
    println!("{} {} {} {}", hello, world, whole, s);


    let res = first_world(&s);
    // println!("{}", res); // 5 返回usize时
    println!("{}", res); // hello 返回slice时
    // 这清空了字符串，使其等于 ""
    s.clear();
}
/// 第一个单词
// 这种方式没有切割字符串，而是返回一个字符串的索引，当字符串发生改变，在通过索引就会存在问题
// fn first_world(s: &String) -> usize {
// “字符串 slice” 的类型声明写作 &str：
fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}