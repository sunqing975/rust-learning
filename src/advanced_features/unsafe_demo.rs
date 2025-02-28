use std::slice;

pub fn unsafe_demo() {
    let mut num = 5;
    // 可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
    // as 将不可变和可变引用强转为对应的裸指针类型
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

// 不安全函数和方法与常规函数方法十分类似，除了其开头有一个额外的 unsafe。
unsafe fn dangerous() {}

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
// let len = values.len();
// assert!(mid <= len);
// second mutable borrow occurs here
// (&mut values[..mid], &mut values[mid..])
// }

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // 使用 as_mut_ptr 方法访问 slice 的裸指针。
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            // slice::from_raw_parts_mut 函数获取一个裸指针和一个长度来创建一个 slice。
            slice::from_raw_parts_mut(ptr, mid),
            // ptr 上调用 add 方法并使用 mid 作为参数来获取一个从 mid 开始的裸指针
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
// extern，有助于创建和使用 外部函数接口（Foreign Function Interface，FFI）。
// 外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数。
extern "C" {
    fn abs(input: i32) -> i32;
}

// 也可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口。不同于创建整个 extern 块，
// 就在 fn 关键字之前增加 extern 关键字并为相关函数指定所用到的 ABI。还需增加 #[no_mangle] 注解来告诉 Rust 编译器不要 mangle 此函数的名称。
// extern 的使用无需 unsafe。
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// 不过这对于 Rust 的所有权规则来说是有问题的。如果有两个线程访问相同的可变全局变量，则可能会造成数据竞争。
// 全局变量在 Rust 中被称为 静态（static）变量。
// 访问不可变静态变量是安全的。
static HELLO_WORLD: &str = "Hello, world!";

// 常量与不可变静态变量的一个微妙的区别是静态变量中的值有一个固定的内存地址。使用这个值总是会访问相同的地址。
// 另一方面，常量则允许在任何被用到的时候复制其数据。另一个区别在于静态变量可以是可变的。访问和修改可变静态变量都是 不安全 的。
// 拥有可以全局访问的可变数据，难以保证不存在数据竞争，这就是为何 Rust 认为可变静态变量是不安全的。
// 任何可能的情况，请优先使用第十六章讨论的并发技术和线程安全智能指针，这样编译器就能检测不同线程间的数据访问是否是安全的。
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe 的另一个操作用例是实现不安全 trait。当 trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时 trait 是不安全的。
// 可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // 和引用一样，裸指针是不可变或可变的，分别写作 *const T 和 *mut T。
        // 这里的星号不是解引用运算符；它是类型名称的一部分。在裸指针的上下文中，不可变 意味着指针解引用之后不能直接赋值。
        // 裸指针与引用和智能指针的区别在于
        // 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
        // 不保证指向有效的内存
        // 允许为空
        // 不能实现任何自动清理功能
        // 通过去掉 Rust 强加的保证，你可以放弃安全保证以换取性能或使用另一个语言或硬件接口的能力，此时 Rust 的保证并不适用。
        unsafe_demo();
    }

    #[test]
    fn it_works_2() {
        // 创建一个指向任意内存地址的裸指针。尝试使用任意内存是未定义行为：
        // 此地址可能有数据也可能没有，编译器可能会优化掉这个内存访问，或者程序可能会出现段错误（segmentation fault）。
        let address = 0x012345usize;
        let r = address as *const i32;
    }

    #[test]
    fn it_works_3() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        // 不能 解引用 裸指针和读取其指向的数据。现在我们要做的就是对裸指针使用解引用运算符 *，这需要一个 unsafe 块
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    #[test]
    fn it_works_4() {
        // 第二类可以在不安全块中进行的操作是调用不安全函数。
        unsafe {
            dangerous();
        }
    }

    #[test]
    fn it_works_5() {
        // 仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。
        // 事实上，将不安全代码封装进安全函数是一个常见的抽象。
        //
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    #[test]
    fn it_works_6() {
        let address = 0x01234usize;
        let r = address as *mut i32;
        let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    }

    #[test]
    fn it_works_7() {
        println!("name is: {HELLO_WORLD}");
    }

    #[test]
    fn it_works_8() {
        add_to_count(3);
        // 使用 mut 关键来指定可变性。任何读写 COUNTER 的代码都必须位于 unsafe 块中。
        // 这段代码可以编译并如期打印出 COUNTER: 3，因为这是单线程的。拥有多个线程访问 COUNTER 则可能导致数据竞争。
        unsafe {
            assert_eq!(unsafe { COUNTER }, 3);
        }
    }
}
