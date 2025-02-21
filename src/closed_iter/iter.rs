use std::iter::Iterator;

fn iter_def() {
    // 迭代器（iterator）负责遍历序列中的每一项并确定序列何时结束的逻辑。
    // 在 Rust 中，迭代器是 惰性的（lazy），这意味着在调用消费迭代器的方法之前不会执行任何操作。
    let mut v = vec![1, 2, 3, 4, 5];
    // 代码本身并没有执行任何有用的操作。
    let iter = v.iter();

    for val in iter {
        println!("Got: {val}");
    }
}

// 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：
// pub trait Iterator {
//     // 新语法：type Item 和 Self::Item，它们定义了 trait 的 关联类型（associated type）。
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use std::iter::Iterator;
    use super::*;
    #[test]
    fn test_iter_def() {
        iter_def()
    }

    #[test]
    fn test_iter_next() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();
        // 从 next 调用中获取的值是对 vector 中值的不可变引用。iter 方法生成一个不可变引用的迭代器。
        // 如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter。
        // 类似地，如果我们希望迭代可变引用，可以调用 iter_mut 而不是 iter。
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn test_iter_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        // 调用 next 方法的方法被称为 消费适配器（consuming adaptors），因为调用它们会消耗迭代器。一个消费适配器的例子是 sum 方法。
        // 这个方法获取迭代器的所有权并反复调用 next 来遍历迭代器，因而会消费迭代器。
        // 在遍历过程中，它将每个项累加到一个总和中，并在迭代完成时返回这个总和。
        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn test_iter_adaptors() {
        let v1: Vec<i32> = vec![1, 2, 3];
        // 迭代器适配器（iterator adaptors），它们不会消耗当前的迭代器，而是通过改变原始迭代器的某些方面来生成不同的迭代器。类似java中的stream
        // collect 这个方法消费迭代器并将结果收集到一个集合数据类型中。
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn test_iterator_closure() {
        // 很多迭代器适配器接受闭包作为参数，而我们通常会指定捕获其环境的闭包作为迭代器适配器的参数。
        let v1: Vec<i32> = vec![1, 2, 3];
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}