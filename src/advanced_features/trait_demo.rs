pub fn trait_demo() {}

pub trait Iterator {
    // 关联类型（associated types）让我们可以在 trait 里面增加一个待定义的类型（类型占位符），将类型占位符与 trait 相关联，
    // 这样 trait 的方法签名中就可以使用这些占位符类型。trait 的实现者在实现这个 trait 的时候，会指定一个具体类型，来替换掉这个占位符。
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        trait_demo()
    }
}
