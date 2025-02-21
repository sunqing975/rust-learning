
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // 对于 if，表达式必须返回一个布尔值，而match它可以是任何类型的
    // 一个分支有两个部分：一个模式和一些代码。第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。
    // 这里的代码就仅仅是值 1。每一个分支之间使用逗号分隔。
    // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。

    // 如果想要在分支中运行多行代码，可以使用大括号，而分支后的逗号是可选的。
    // 否则逗号必有
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

/// 匹配
pub fn match_def() {
    // 对于 if，表达式必须返回一个布尔值，而match它可以是任何类型的

    let some_u8_value = Some(0);
    let a = 15;

    let i = value_in_cents(Coin::Penny);
    println!("i is {}", i);

    let i = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("i is {}", i)
}

/// 匹配Option
pub fn option_match() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six is {:?}", six);
    let none = plus_one(None);
    println!("none is {:?}", none)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // 匹配是穷尽的
        None => None,
        Some(i) => Some(i + 1)
    }
}


/// 匹配其他模式
pub fn match_other() {
    let x = 8;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        // 变量 `other` 从未使用 ,可以在前面加上下划线(_other)就不会提示问题了
        _other => println!("anything"),
    }

    // 当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    match x {
        1..=5 => println!("one through five"),
        _ => (),
    }
}