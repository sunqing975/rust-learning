// Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
// 我们更倾向于称之为 never type。这个名字描述了它的作用：在函数从不返回的时候充当返回值。
// 从不返回的函数被称为 发散函数（diverging functions）。不能创建 ! 类型的值，所以 bar 也不可能返回值。
// fn bar() -> ! {
//     // --snip--
// }

// 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；
// 相反它把控制权交回上层循环，所以在 Err 的情况，事实上并未对 guess 赋值。

// never type 的另一个用途是 panic!。还记得 Option<T> 上的 unwrap 函数吗？它产生一个值或 panic。
// 最后一个有着 ! 类型的表达式是 loop：
//     print!("forever ");
//
//     loop {
//         print!("and ever ");
//     }
fn never_type() {
    let guess = "3";
    let guess:i32 = match guess.trim().parse() {
        Ok(m) =>  m,
        Err(_) => panic!("not a number")
        // match 的分支必须返回相同的类型。
        // Err(_) => "hello",
        // continue 的值是 !。也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。
        // 前者是 u32 值，而后者是 ! 值。因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。
        // Err(_) => continue,
    };
}
