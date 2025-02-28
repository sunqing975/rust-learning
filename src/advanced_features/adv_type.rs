// 哪里混用 Kilometers 和 i32 的值，编译器也不会给出一个错误。
// 类型别名的主要用途是减少重复。
// 类型别名也经常与 Result<T, E> 结合使用来减少重复。
type Kilometers = i32;

#[cfg(test)]
mod tests {
    use crate::advanced_features::adv_type::Kilometers;

    #[test]
    fn it_works() {
        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }

    #[test]
    fn it_works2() {
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {
            // --snip--
        }

        fn returns_long_type() -> Thunk {
            // --snip--
            Box::new(|| ())
        }
    }
}
