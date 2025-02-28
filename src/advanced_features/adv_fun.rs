fn fn_point() {
    // 向函数传递闭包；也可以向函数传递常规函数！
    // 函数满足类型 fn（小写的 f），不要与闭包 trait 的 Fn 相混淆。fn 被称为 函数指针（function pointer）。
    // 不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。
    // 函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数

    // 一个只期望接受 fn 而不接受闭包的情况的例子是与不存在闭包的外部代码交互时：C 语言的函数可以接受函数作为参数，但 C 语言没有闭包。
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// 闭包表现为 trait，这意味着不能直接返回闭包。对于大部分需要返回 trait 的情况，
// 可以使用实现了期望返回的 trait 的具体类型来替代函数的返回值。但是这不能用于闭包，
// 因为它们没有一个可返回的具体类型；例如不允许使用函数指针 fn 作为返回值类型。
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Rust 没有反射的能力，因此其无法在运行时获取类型名。
#[derive(HelloMacro)]
struct Pancakes;

// derive 只能用于结构体和枚举；
#[derive(HelloMacro)]
struct A;

// 类属性宏
// 类属性宏与自定义派生宏相似，
// 不同的是 derive 属性生成代码，它们（类属性宏）能让你创建新的属性。它们也更为灵活；
// derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数。
// #[route(GET, "/")]
// fn index() {}

// 类函数宏
// 类函数（Function-like）宏的定义看起来像函数调用的宏。类似于 macro_rules!，它们比函数更灵活；例如，可以接受未知数量的参数。


fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[cfg(test)]
mod tests {
    use hello_macro::HelloMacro;
    use crate::advanced_features::adv_fun::{fn_point, Pancakes, A};

    #[test]
    fn it_works() {
        fn_point()
    }

    #[test]
    fn test_closure() {
        Pancakes::hello_macro();
        A::hello_macro();
    }
}
