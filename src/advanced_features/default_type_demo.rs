// Rhs=Self：这个语法叫做 默认类型参数（default type parameters）。Rhs 是一个泛型类型参数（“right hand side” 的缩写），
// 它用于定义 add 方法中的 rhs 参数。如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。
// trait Add<Rhs = Self> {
//     type Output;
//
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 当为 Point 实现 Add 时，使用了默认的 Rhs，因为我们希望将两个 Point 实例相加。
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// 将现有类型简单封装进另一个结构体的方式被称为 newtype 模式（newtype pattern)
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // 为了使 Millimeters 和 Meters 能够相加，我们指定 impl Add<Meters> 来设定 Rhs 类型参数的值而不是使用默认的 Self。
    //
    // 默认参数类型主要用于如下两个方面：
    //
    // 扩展类型而不破坏现有代码。
    // 在大部分用户都不需要的特定情况进行自定义。
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub fn default_type_demo() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        default_type_demo();
    }
}
