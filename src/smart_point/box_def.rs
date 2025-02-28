use std::mem;
use std::ops::Deref;
use self::List::{Cons, Nil};

/// 创建一个box
pub fn box_demo() {
    let a = Box::new(5);
    println!("a is {}", a);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("Size of Message: {} bytes", size_of::<Message2>());

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // type `MyBox<{integer}>` cannot be dereferenced 当MyBox未实现 deref trait时
    assert_eq!(5, *y);
    println!("*y is {}", *y);
    deref_coercions();
}


/// 递归类型（recursive type）的值可以拥有另一个同类型的值作为其自身的一部分。
enum List {
    // box 只提供了间接存储和堆分配；它们并没有任何其他特殊的功能，
    // Box<T> 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待。
    // 当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Move {
    x: i32,
    y: i32,
}
struct Message2 {
    quit: (),
    move_def: Move,
    write: String,
    change_color: (i32, i32, i32),
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 没有 Deref trait 的话，编译器只会解引用 & 引用类型。
// deref 方法向编译器提供了获取任何实现了 Deref trait 的类型的值，并且调用这个类型的 deref 方法来获取一个它知道如何解引用的 & 引用的能力。
// deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。
// 如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权。
// 注意，每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换
impl<T> Deref for MyBox<T> {
    // type Target = T; 语法定义了用于此 trait 的关联类型。关联类型是一个稍有不同的定义泛型参数的方式
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // deref 方法体中写入了 &self.0，这样 deref 返回了我希望通过 * 运算符访问的值的引用。
        &self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_coercions() {
    let m = MyBox::new(String::from("Rust"));
    // 这里使用 &m 调用 hello 函数，其为 MyBox<String> 值的引用。因为示例 15-10 中在 MyBox<T> 上实现了 Deref trait，
    // Rust 可以通过 deref 调用将 &MyBox<String> 变为 &String。
    // 标准库中提供了 String 上的 Deref 实现，其会返回字符串 slice，这可以在 Deref 的 API 文档中看到。
    // Rust 再次调用 deref 将 &String 变为 &str，这就符合 hello 函数的定义了。
    hello(&m);
}

fn deref_def() {
    // 使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。
    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
    // 头两个情况除了第二种实现了可变性之外是相同的：第一种情况表明如果有一个 &T，而 T 实现了返回 U 类型的 Deref，则可以直接得到 &U。第二种情况表明对于可变引用也有着相同的行为。
    // 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。
    // 因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。
    // 将一个可变引用转换为不可变引用永远也不会打破借用规则。
    // 将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。
    // 因此，Rust 无法假设将不可变引用转换为可变引用是可能的。
    println!("Hello Rust")
}
