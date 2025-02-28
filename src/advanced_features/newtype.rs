use std::fmt;

// 孤儿规则（orphan rule），它说明只要 trait 或类型对于当前 crate 是本地的话就可以在此类型上实现该 trait。
// 一个绕开这个限制的方法是使用 newtype 模式（newtype pattern）
pub fn newtype() {
    // 此方法的缺点是，因为 Wrapper 是一个新类型，它没有定义于其值之上的方法；必须直接在 Wrapper 上实现 Vec<T> 的所有方法，
    // 这样就可以代理到self.0 上 —— 这就允许我们完全像 Vec<T> 那样对待 Wrapper。如果希望新类型拥有其内部类型的每一个方法，
    // 为封装类型实现 Deref trait（第十五章 “通过 Deref trait 将智能指针当作常规引用处理” 部分讨论过）并返回其内部类型是一种解决方案
    // 如果不希望封装类型拥有所有内部类型的方法 —— 比如为了限制封装类型的行为 —— 则必须只自行实现所需的方法。
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use crate::advanced_features::newtype::newtype;

    #[test]
    fn it_works() {
        newtype();
    }
}
