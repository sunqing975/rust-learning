enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct IpAddr2 {
    kind: IpAddrKind,
    address: (u8, u8, u8, u8),
}

enum IpAddress {
    V4(String),
    V6(String),
}

/// 定义一个函数enum_def，用于演示枚举类型IpAddrKind的使用
pub fn enum_def() {
    // 创建一个IpAddrKind类型的V4实例
    let four = IpAddrKind::V4;
    // 创建一个IpAddrKind类型的V6实例
    let six = IpAddrKind::V6;

    // 调用route函数处理IPv4类型的地址
    route(four);
    // 调用route函数处理IPv6类型的地址
    route(six);
    // 直接调用route函数处理IPv6类型的地址，展示不同方式的函数调用
    route(IpAddrKind::V6);


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };


    let home = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    let home = IpAddr2 {
        kind: IpAddrKind::V4,
        address: (127, 0, 0, 1),
    };
}


fn route(ip_type: IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}

// 枚举中的元素可以认为是结构体，枚举本身可以看做是多个结构体的集合。
// Quit 是一个类单元结构体，所以它不需要任何数据来创建一个 Quit 实例。
// Move 是一个结构体。
// Write 是一个元组结构体，它只有一个字段。
// ChangeColor 是一个元组结构体，它有三个字段。
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        println!("{:?}-----Message::call", self)
    }
}

/// 定义一个函数enum_struct，用于演示枚举类型Message的使用
pub fn enum_struct() {
    Message::Quit.call();
    Message::Move { x: 1, y: 2 }.call();
    Message::Write(String::from("hello")).call();
    Message::ChangeColor(1, 2, 3).call();
}