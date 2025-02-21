/// if let
pub fn if_let() {
    // if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况。
    let config_max = Some(3u8);
    // 如果值是 Some，我们希望打印出 Some 成员中的值，这个值被绑定到模式中的 max 变量里。对于 None 值我们不希望做任何操作。
    // 为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => ()，这样也要增加很多烦人的样板代码。
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured")
    }
}