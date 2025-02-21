/// 测试 panic!
pub fn panic_def() {
    // Rust 将错误分为两大类：可恢复的（recoverable）和 不可恢复的（unrecoverable）错误。
    // Result<T, E> 类型，用于处理可恢复的错误，还有 panic! 宏，在程序遇到不可恢复的错误时停止执行。
    // 通常情况下这些 panic 会打印出一个错误信息，展开并清理栈数据，然后退出。
    // 通过一个环境变量，你也可以让 Rust 在 panic 发生时打印调用堆栈（call stack）以便于定位 panic 的原因。


    // 当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
    // 另一种选择是直接 终止（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。
    // 如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。
    // 例如，如果你想要在 release 模式中 panic 时直接终止：
    // [profile.release]
    // panic = 'abort'

    // 显式调用 panic!
    // panic!("crash and burn");

    let arr = vec![1, 2, 3];
    arr[99];
}