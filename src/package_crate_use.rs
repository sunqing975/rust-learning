mod back_of_house {
    // 这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例 (这里我们命名为 summer)。
    // 如果 Breakfast 没有这样的函数，我们将无法在 eat_at_restaurant 中创建 Breakfast 实例，
    // 因为我们不能在 eat_at_restaurant 中设置私有字段 seasonal_fruit 的值。
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // 如果我们将枚举设为公有，则它的所有成员都将变为公有。
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_at_restaurant() {
    // 创建一个变量，并使用结构体初始化它。
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变变量的值
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 不能修改结构体的字段，因为其字段是私有的。
    // meal.seasonal_fruit = String::from("blueberries");

    //创建一个变量，并使用结构体初始化它。
}


pub mod a {
    pub mod b {
        pub fn test_use_() {}
    }
}

// 注意 use 只能创建 use 所在的特定作用域内的短路径。
// use crate::package_crate_use::a::b;
mod customer {
    // 将 use 移动到 customer 模块内，
    use crate::package_crate_use::a::b;
    pub fn test_use_() {
        // use of undeclared crate or module `b`
        b::test_use_();
        // 或者在子模块 customer 内通过 super::a::b 引用父模块中的这个短路径。
        super::a::b::test_use_();
    }
}


use std::fmt::Result;
// 使用 as 指定一个新的本地名称或者别名。
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}