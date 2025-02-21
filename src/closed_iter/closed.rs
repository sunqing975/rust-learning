fn closures_def() {
    // 闭包
    // Rust 的 闭包（closures）是可以保存在变量中或作为参数传递给其他函数的匿名函数。
    // 闭包通常不要求像 fn 函数那样对参数和返回值进行类型注解。函数需要类型注解是因为这些类型是暴露给用户的显式接口的一部分。
    // 严格定义这些接口对于确保所有人对函数使用和返回值的类型达成一致理解非常重要。
    // 与此相比，闭包并不用于这样暴露在外的接口：它们储存在变量中并被使用，不用命名它们或暴露给库的用户调用。

    // 在这些有限的上下文中，编译器可以推断参数和返回值的类型，类似于它推断大多数变量类型的方式（尽管在某些罕见的情况下，编译器也需要闭包的类型注解）。

    //对于闭包定义，编译器会为每个参数和返回值推断出一个具体类型。

    let add_one = |x| x + 1;

    // 闭包可以通过三种方式捕获其环境中的值，它们直接对应到函数获取参数的三种方式：不可变借用、可变借用和获取所有权。
    println!("{}", add_one(1));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[cfg(test)]
mod tests {
    use std::thread;
    use crate::closed_iter::closed::Rectangle;

    #[test]
    fn test_closures_def() {
        let add_one = |x| x;
        // 第一次使用 String 值调用 example_closure 时，编译器推断出 x 的类型以及闭包的返回类型为 String。
        // 接着这些类型被锁定进闭包 example_closure 中，如果尝试对同一闭包使用不同类型则就会得到类型错误。
        assert_eq!(add_one(1), 2);
        // assert_eq!(add_one("12"), 5);
    }


    #[test]
    fn test_closures_refer() {
        let list = vec![1, 2, 3];
        // 一个捕获名为 list 的 vector 的不可变引用的闭包，因为只需不可变引用就能打印其值：
        let only_borrows = || println!("From closure: {:?}", list);
        println!("Before calling closure {:?}", list);
        only_borrows();
        println!("After calling closure {:?}", list);
    }

    #[test]
    fn test_closures_mutable() {
        let mut list = vec![1, 2, 3];
        println!("Before calling closure {:?}", list);
        // 定义并调用一个捕获可变引用的闭包
        let mut borrows_mutably = || list.push(7);
        // borrows_mutably 被定义时，它捕获了对 list 的可变引用。闭包在被调用后就不再被使用，这时可变借用结束。
        // 因为当可变借用存在时不允许有其它的借用，所以在闭包定义和调用之间不能有不可变引用来进行打印。
        // immutable borrow occurs here
        // println!("Before calling closure {:?}", list);
        borrows_mutably();
        println!("After calling closure {:?}", list);
    }

    #[test]
    fn test_closures_move() {
        // move 关键字 强制闭包获取它在环境中所使用的值的所有权
        // 当将闭包传递到一个新的线程时，这个技巧特别有用，因为它将数据的所有权移动到新线程中。
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        thread::spawn(move || println!("From thread: {list:?}"))
            .join().unwrap();

        // value borrowed here after move
        // println!("After defining closure: {list:?}");
    }


    #[test]
    fn test_closures_capture_fn_mut() {
        // 闭包体可以执行以下任一操作：
        // 将一个捕获的值移出闭包，
        // 修改捕获的值，
        // 既不移动也不修改值，
        // 或者一开始就不从环境中捕获任何值。
        // FnOnce 适用于只能被调用一次的闭包。所有闭包至少都实现了这个 trait，因为所有闭包都能被调用。一个会将捕获的值从闭包体中移出的闭包只会实现 FnOnce trait，而不会实现其他 Fn 相关的 trait，因为它只能被调用一次。
        // FnMut 适用于不会将捕获的值移出闭包体，但可能会修改捕获值的闭包。这类闭包可以被调用多次。
        // Fn 适用于既不将捕获的值移出闭包体，也不修改捕获值的闭包，同时也包括不从环境中捕获任何值的闭包。这类闭包可以被多次调用而不会改变其环境，这在会多次并发调用闭包的场景中十分重要。

        // 函数也可以实现所有的三种 Fn traits。如果我们要做的事情不需要从环境中捕获值，则可以在需要某种实现了 Fn trait 的东西时使用函数而不是闭包。
        // 举个例子，可以在 Option<Vec<T>> 的值上调用 unwrap_or_else(Vec::new)，以便在值为 None 时获取一个新的空的 vector。
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];
        // sort_by_key 被定义为接收一个 FnMut 闭包的原因是它会多次调用这个闭包：对 slice 中的每个元素调用一次。
        // 闭包 |r| r.width 不捕获、修改或将任何东西移出它的环境，所以它满足 trait bound 的要求。
        list.sort_by_key(|r| r.width);
        println!("{list:#?}");
    }

    #[test]
    fn test_closures_capture_fn_once() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        // let mut sort_operations = vec![];
        let value = String::from("closure called");

        list.sort_by_key(|r| {
            // 这个闭包只能被调用一次；尝试第二次调用它将无法工作，因为这时 value 已经不在闭包的环境中，无法被再次插入 sort_operations 中！
            // 因而，这个闭包只实现了 FnOnce。
            // sort_operations.push(value);
            r.width
        });
        println!("{list:#?}");
    }

    #[test]
    fn test_closures_capture_fn_mut_count() {
        let mut list = [
            Rectangle { width: 10, height: 1 },
            Rectangle { width: 3, height: 5 },
            Rectangle { width: 7, height: 12 },
        ];

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{list:#?}, sorted in {num_sort_operations} operations");
    }
}