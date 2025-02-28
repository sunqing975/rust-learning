pub fn drop_demo() {
    // Drop，其允许我们在值要离开作用域时执行一些代码。
    // 被用于释放类似于文件或网络连接的资源。
    println!("drop_demo");
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // explicit destructor calls not allowed
    // 错误信息使用了术语 析构函数（destructor），这是一个清理实例的函数的通用编程概念。
    // 析构函数 对应创建实例的 构造函数。Rust 中的 drop 函数就是这么一个析构函数。
    // Rust 不允许我们显式调用 drop 因为 Rust 仍然会在 main 的结尾对值自动调用 drop，这会导致一个 double free 错误，因为 Rust 会尝试清理相同的值两次。

    // c.drop();
    // 因为不能禁用当值离开作用域时自动插入的 drop，并且不能显式调用 drop，如果我们需要强制提早清理值，可以使用 std::mem::drop 函数。
    // std::mem::drop 函数不同于 Drop trait 中的 drop 方法。可以通过传递希望强制丢弃的值作为参数。std::mem::drop 位于 prelude
    drop(c);
    // 我们也无需担心意外的清理掉仍在使用的值，这会造成编译器错误：所有权系统确保引用总是有效的，也会确保 drop 只会在值不再被使用时被调用一次。
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // Rust 并不允许我们主动调用 Drop trait 的 drop 方法；
    // 当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop。
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

