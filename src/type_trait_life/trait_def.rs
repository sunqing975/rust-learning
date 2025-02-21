use std::fmt::Display;

// 在方法签名后跟分号，而不是在大括号中提供其实现。
// 接着每一个实现这个 trait 的类型都需要提供其自定义行为的方法体，
// 编译器也会确保任何实现 Summary trait 的类型都拥有与这个签名的定义完全一致的 summarize 方法。
// trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。
/// 定义一个函数trait_def，用于演示trait的使用
pub trait Summary {
    fn summarize(&self) -> String;

    // 有时为 trait 中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为是很有用的。
    // 这样当为某个特定类型实现 trait 时，可以选择保留或重载每个方法的默认行为。
    fn print_content(&self) {
        println!("11111")
    }
}

/// 定义一个函数trait_def，用于演示trait的使用
pub struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

/// 定义一个函数trait_def，用于演示trait的使用
pub struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.username)
    }
}

impl Tweet {
    pub fn new(username: String, content: String) -> Tweet {
        Tweet {
            username: username,
            content: content,
            reply: false,
            retweet: false,
        }
    }
}

/// 定义一个函数trait_def，用于演示trait的使用
pub fn trait_def() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

// trait 作为参数
// impl Trait 很方便，适用于短小的例子。更长的 trait bound 则适用于更复杂的场景。例如，可以获取两个实现了 Summary 的参数。
/// 泛型函数
pub fn trait_as_param() {
    let tweet = Tweet::new(String::from("horse_ebooks"), String::from("of course, as you probably already know, people"));
    // 任何用其它如 String 或 i32 的类型调用该函数的代码都不能编译，因为它们没有实现 Summary。
    notify(&tweet);

    notify_bound(&tweet);
}
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。
// 这可以通过 + 语法实现：
fn notify_add(item: &(impl Summary + Display)) {}
fn notify_bound_add<T: Summary + Display>(item: &T) {}

fn notify_bound_where_no_res<T, U>(t: &T, u: &U)
where
    T: Summary,
    U: Display,
{}
fn notify_bound_where<T, U>(t: &T, u: &U) -> i32
where
    T: Summary,
    U: Display,
{ 1 }

// 返回实现了 trait 的类型
// 在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型：
// 返回一个只是指定了需要实现的 trait 的类型的能力在闭包和迭代器场景十分的有用
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// `if` and `else` have incompatible types if-else返回的是不相容的类型，无法编译通过
// 这里尝试返回 NewsArticle 或 Tweet。这不能编译，因为 impl Trait 工作方式的限制。
// fn returns_summarizable_non_type(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }





struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// 使用 trait bound 有条件地实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}



// 对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations，它们被广泛的用于 Rust 标准库中。
// 例如，标准库为任何实现了 Display trait 的类型实现了 ToString trait。这个 impl 块看起来像这样：
// impl<T: Display> ToString for T {
//     fn to_string(&self) -> String {
//         todo!()
//     }
// }


