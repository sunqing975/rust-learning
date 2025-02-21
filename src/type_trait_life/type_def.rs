// 在 Rust 中，Vec<String> 类型的参数可以直接传入接受 &[String] 参数的函数中，这是因为 Rust 提供了自动引用和解引用（Deref coercion）机制，以及切片（slice）和向量（vector）之间的转换支持。
//
// 切片与向量的关系
// 切片 (&[T])：是一个动态大小类型（DST, Dynamically Sized Type），表示一个连续的、不可变的元素序列。它不拥有数据的所有权，而是借用数据。
// 向量 (Vec<T>)：是一个拥有所有权的动态数组，可以动态增长或缩小。
// 由于 Vec<T> 内部存储的数据是连续的，并且可以通过切片来借用这些数据，因此 Vec<T> 可以很方便地转换为 &[T]。
//
// 自动引用和解引用
// Rust 的自动引用和解引用机制允许你隐式地进行某些类型的转换。具体来说，当你传递一个 Vec<String> 给一个期望 &[String] 的函数时，Rust 会自动将 Vec<String> 转换为 &[String]。

// 切片的定义：切片是一种借用，它可以引用数组、向量或其他集合类型的一部分数据。
// 切片的类型：包括不可变切片 &[T] 和可变切片 &mut [T]。
// 切片的创建：可以通过范围语法 [start..end] 来创建切片。
// 切片的操作：可以遍历、获取长度、检查是否为空等。
// 函数参数中的切片：切片常用于函数参数中，使得函数可以操作传入的集合数据，而无需拥有其所有权。
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = &list[0];
    for x in list {
        if largest < x {
            largest = x;
        }
    }
    *largest
}
/// 获取最大值
pub fn get_largest() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let b = largest(&a);
    println!("{}", b);

    let c = largest(&[1, 2, 3, 40, 5, 6, 7, 8, 9, 10]);
    println!("{}", c);
}


// Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

// 定义方法时也可以为泛型指定限制（constraint）
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, point2: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: point2.y,
        }
    }
}

/// 定义结构体
pub fn struct_type() {
    let p1 = Point { x: 1, y: 2 };
    println!("{}-{}", p1.x, p1.y);

    println!("{}", p1.get_x());

    let p2 = Point { x: 1.1, y: 2.2 };
    println!("{}-{}", p2.x, p2.y);
    println!("{}", p2.distance_from_origin());

    let p3 = Point2 { x: 1, y: 2.2 };
    println!("{}-{}", p3.x, p3.y);
    let p4 = Point2 { x: 1.1, y: 2 };
    println!("{}-{}", p4.x, p4.y);

    println!("{:?}", p3.mixup(p4))
}


