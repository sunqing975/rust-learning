use std::fmt::Display;
use std::string;

/// 生命周期
pub fn lifetime_demo() {
    // Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，
    // 正如大部分时候类型也是可以推断的一样。类似于当因为有多种可能类型的时候必须注明类型，也会出现引用的生命周期以一些不同方式相关联的情况，
    // 所以 Rust 需要我们使用泛型生命周期参数来注明它们的关系，这样就能确保运行时实际使用的引用绝对是有效的。

    // cmp_str_len();

    // lifetime_struct();

    lifetime_elision();

    // 'static，其生命周期能够存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期
    let s: &'static str = "I have a static lifetime.";
}

fn first_demo() {
    // 生命周期的主要目标是避免悬垂引用（dangling references），后者会导致程序引用了非预期引用的数据。
    // let r;
    // {
    // let x = 5;
    // borrowed value does not live long enough
    // r = &x;
    // }
    // println!("r: {r}");
}

// 借用检查器
fn borrow_checker() {
    let x = 5;             // ----------+-- 'b
    //           |
    let r = &x;           // --+-- 'a  |
    //   |       |
    println!("r: {r}");         //   |       |
    // --+       |
}                               // ----------+


fn cmp_str_len() {
    let a = String::from("hello");
    let b = String::from("world");
    let c = longest(a.as_str(), b.as_str());
    println!("The longest string is {}", c);
}
// 泛型生命周期参数
// -> &str : expected named lifetime parameter
// 因为 Rust 并不知道将要返回的引用是指向 x 或 y。
// 生命周期注解并不改变任何引用的生命周期的长短。
// 相反它们描述了多个引用生命周期相互的关系，而不影响其生命周期。
// 生命周期参数名称必须以撇号（'）开头，其名称通常全是小写，类似于泛型其名称非常短。大多数人使用 'a 作为第一个生命周期注解。
// 生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开。

// 加上生命周期参数时，这两个参数和返回的引用存活的一样久。按照短的生命周期的引用为准，保证在这个生命周期范围内引用都是有效的。
// 实际含义是 longest 函数返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致。
// 当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
// 换一种说法就是泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个。
// 因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
// longest 函数返回的引用的生命周期应该与传入参数的生命周期中较短那个保持一致。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// 定义包含引用的结构体，不过这需要为结构体定义中的每一个引用添加生命周期注解。
// 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn lifetime_elision() {
    // 生命周期省略规则
    // 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。

    // 编译器采用三条规则来判断引用何时不需要明确的注解。第一条规则适用于输入生命周期，后两条规则适用于输出生命周期。
    // 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
    // 第一条规则是编译器为每一个引用参数都分配一个生命周期参数。(适用于输入生命周期)
    // 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数。(适用于输出生命周期)
    // 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
    //      说明是个对象的方法 (method)，那么所有输出生命周期参数被赋予 self 的生命周期。(适用于输出生命周期)

    let s = String::from("hello world");
    let word = first_word(&s[0..6]);
    println!("The first word is: {}", word)
}

//
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}