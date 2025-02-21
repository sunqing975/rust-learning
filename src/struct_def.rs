struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

/// 结构体
pub fn struct_def() {
    // 实例化结构体
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 可变结构体
    // 注意所有权的移动，所以在 user2 中，user的username 和 email 的所有权已经移动到 user2 中。
    let mut user2 = User {
        email: String::from("sometwo@example.com"),
        // 结构体更新语法
        ..user
    };
    user2.active = false;
    // value borrowed here after move
    // println!("{}", user.username);
    let user3 = struct_build_user(String::from("some3"), String::from("some3@example.com"));
}

fn struct_build_user(username: String, email: String) -> User {
    // 当变量的名称与结构体字段名称相同时，可以使用简写语法，也可以写完整
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

/*
    元组结构体（tuple structs）。
*/
/// 元组结构体
pub fn tuple_struct() {
    // 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。
    // 当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，
    let black = Color(0, 0, 0);
    println!("{:#?}", black);
    let origin = Point(0, 0, 0);
    println!("{:#?}", origin);
}

// struct 没有字段
// 类单元结构体
// 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
struct AlwaysEqual;
fn struct_no_field() {
    let subject = AlwaysEqual;
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关。
// 不是方法的关联函数经常被用作返回一个结构体新实例的构造函数。这些函数的名称通常为 new ，但 new 并不是一个关键字。
impl Rectangle {
    // 我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
    // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
    // 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；
    // 这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。

    //通常，但并不总是如此，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。
    // 这样的方法被称为 getters，Rust 并不像其他一些语言那样为结构字段自动实现它们。
    // Getters 很有用，因为你可以把字段变成私有的，但方法是公共的，这样就可以把对字段的只读访问作为该类型公共 API 的一部分。

    // Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。方法调用是 Rust 中少数几个拥有这种行为的地方。
    // 它是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：
    //
    //
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // 在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// 方法
pub fn struct_method() {
    let rect1 = Rectangle {
        width: dbg!(30),
        height: 50,
    };
    // 另一种使用 Debug 格式打印数值的方法是使用 dbg! 宏。
    // dbg! 宏接收一个表达式的所有权（与 println! 宏相反，后者接收的是引用），打印出代码中调用 dbg! 宏时所在的文件和行号，
    // 以及该表达式的结果值，并返回该值的所有权。
    // 注意：调用 dbg! 宏会打印到标准错误控制台流（stderr），与 println! 不同，后者会打印到标准输出控制台流（stdout）。
    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );


    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}