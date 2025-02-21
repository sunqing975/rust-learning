use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};

/// 处理`Result`类型的结果，如果文件成功打开，则继续程序的后续操作；
pub fn result_def() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) =>
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Error"),
                }
                _ => panic!("Error"),
            }
    };
}

/// 尝试打开名为"hello.txt"的文件，并处理潜在的错误
///
/// 使用`unwrap`方法来处理`Result`类型的结果，如果文件成功打开，则继续程序的后续操作；
/// 如果文件打开失败（例如文件不存在），则程序会自动终止并返回错误。
pub fn result_unwrap() {
    // 使用unwrap方法直接处理文件打开的结果，避免显式地处理错误
    let f = File::open("hello.txt").unwrap();
}

/// 尝试打开名为"hello.txt"的文件，并返回一个包含文件内容的字符串，如果文件打开失败，则返回相应的错误。
pub fn result_expect() {
    // 使用expect方法来处理文件打开的结果，并在文件打开失败时返回自定义的错误信息
    // 在生产级别的代码中，大部分 Rustaceans 选择 expect 而不是 unwrap 并提供更多关于为何操作期望是一直成功的上下文。
    // 如此如果该假设真的被证明是错的，你也有更多的信息来用于调试。
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

/// 尝试打开名为"hello.txt"的文件，并读取其中的内容，如果文件打开失败，则返回相应的错误。
pub fn result_propagating_panic() {
    let username = read_username_from_file();
    let username = match username {
        Ok(username) => username,
        Err(error) => panic!("Failed to read username: {}", error),
    };
    println!("{}", username);
}

/// 尝试打开名为"hello.txt"的文件，并读取其中的内容，如果文件打开失败，则返回相应的错误。
pub fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => return Err(e),
            },
            _ => return Err(e),
        },
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?
// ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。
// 比如说函数的返回值必须是 Result 才能与这个 return 相兼容。
/// 尝试打开名为"hello.txt"的文件，并读取其中的内容，如果文件打开失败，则返回相应的错误。
pub fn read_username_from_file_question_mark() -> Result<String, std::io::Error> {
    // Result 值之后的 ? 被定义为与示例 9-6 中定义的处理 Result 值的 match 表达式有着完全相同的工作方式。
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    // 如果值是 Err，Err 将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。

    // ? 运算符所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。
    // 当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。
    // 这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

/// 使用标准库提供的 read_to_string 方法
pub fn read_username_from_file_std_lib() -> Result<String, std::io::Error> {
    fs::read_to_string("hello.txt")
}

