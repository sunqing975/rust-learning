trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}

// Rust 就不总是能计算出我们期望的是哪一个类型，除非使用 完全限定语法（fully qualified syntax）。
pub fn same_method() {
    let person = Human;
    //  *waving arms furiously*
    // Human::fly(&person)
    person.fly();

    // 为了能够调用 Pilot trait 或 Wizard trait 的 fly 方法，我们需要使用更明显的语法以便能指定我们指的是哪个 fly 方法。
    Pilot::fly(&person);
    Wizard::fly(&person);

    //不是方法的关联函数没有 self 参数。
    // 当存在多个类型或者 trait 定义了相同函数名的非方法函数时，Rust 就不总是能计算出我们期望的是哪一个类型，除非使用 完全限定语法

    println!("A baby dog is called a {}", Dog::baby_name());

    // cannot call associated function of trait
    // 因为 Animal::baby_name 没有 self 参数，同时这可能会有其它类型实现了 Animal trait，Rust 无法计算出所需的是哪一个 Animal::baby_name 实现。
    // println!("A baby dog is called a {}", Animal::baby_name());
    // 为了消歧义并告诉 Rust 我们希望使用的是 Dog 的 Animal 实现而不是其它类型的 Animal 实现，需要使用 完全限定语法，这是调用函数时最为明确的方式
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        same_method();
    }
}
