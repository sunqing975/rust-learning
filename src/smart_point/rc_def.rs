use std::rc::Rc;
use self::List::{Cons, Nil};

pub fn rc_demo() {
    // 大部分情况下所有权是非常明确的：可以准确地知道哪个变量拥有某个值。然而，有些情况单个值可能会有多个所有者。
    // 例如，在图数据结构中，多个边可能指向相同的节点，而这个节点从概念上讲为所有指向它的边所拥有。
    // 节点在没有任何边指向它从而没有任何所有者之前，都不应该被清理掉。
    // 为了启用多所有权需要显式地使用 Rust 类型 Rc<T>，其为 引用计数（reference counting）的缩写。
    // 引用计数意味着记录一个值的引用数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。


    // 通过不可变引用， Rc<T> 允许在程序的多个部分之间只读地共享数据。
    // 如果 Rc<T> 也允许多个可变引用，则会违反第四章讨论的借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致。不过可以修改数据是非常有用的！
    // 注意 Rc<T> 只能用于单线程场景；

    // Box 智能指针
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // value used here after move
    // let c = Cons(4, Box::new(a));

    // RC 计数指针
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 也可以调用 a.clone() 而不是 Rc::clone(&a)，不过在这里 Rust 的习惯是使用 Rc::clone。
    // Rc::clone 只会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。
    // 通过使用 Rc::clone 进行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。
    // 当查找代码中的性能问题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // 当 c 离开作用域时，计数减 1。
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("rc_demo")
}


enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}
