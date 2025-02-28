use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub mod advanced_features;
pub mod closed_iter;
pub mod collect;
pub mod concurrent;
pub mod enum_match;
pub mod guess;
pub mod hello_world;
pub mod ownership;
pub mod package_crate_use;
pub mod panic;
pub mod pub_use;
pub mod smart_point;
pub mod struct_def;
pub mod type_trait_life;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
// struct Job;

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    //闭包作为参数时可以使用三个不同的 trait：Fn、FnMut 和 FnOnce。
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 如果 Worker 存放的是 Option<thread::JoinHandle<()>，就可以在 Option 上调用 take 方法将值从 Some 成员中移动出来而对 None 成员不做处理。
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub mod unit_test {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    #[cfg(test)]
    mod test {
        use super::*;
        // 测试社区中一直存在关于是否应该对私有函数直接进行测试的论战，而在其他语言中想要测试私有函数是一件困难的，甚至是不可能的事。
        // 不过无论你坚持哪种测试意识形态，Rust 的私有性规则确实允许你测试私有函数。
        #[test]
        fn test_add_success() {
            assert_eq!(add(1, 2), 3);
        }

        #[test]
        fn test_add_fair() {
            assert_eq!(add(1, 2), 4);
        }

        #[test]
        #[ignore]
        fn test_add_ignore() {}

        #[test]
        #[should_panic(expected = "test panic")]
        fn test_add_panic() {
            panic!("test panic")
        }

        #[test]
        fn test_subtract_success() {
            assert_eq!(subtract(1, 2), -1);
        }
    }
}

pub mod base_theory {
    pub mod variable {
        /// 变量默认不可变，mut关键字可以使变量可变
        pub fn variable_immutable() {
            let x = 5;
            println!("x = {}", x);
            // cannot assign twice to immutable variable
            // x = 6;

            let mut y = 1;
            y = 2;
        }

        /// 常量
        pub fn constants() {
            // 不允许对常量使用 mut。常量不光默认不可变，它总是不可变。声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型。
            const MAX_POINTS: i32 = 100_000;
            println!("MAX_POINTS = {}", MAX_POINTS);
        }

        /// 变量遮蔽
        pub fn shadowing() {
            // 第二个变量“遮蔽”了第一个变量，此时任何使用该变量名的行为中都会视为是在使用第二个变量，直到第二个变量自己也被隐藏或第二个变量的作用域结束。
            // 可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏
            // 隐藏与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
            // mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
            let x = 5;
            let x = x + 1;
            println!("x = {}", x);
            let x = "   ";
            let x = x.len();
            println!("x = {}", x);
        }
    }

    pub mod data_types {
        /// 数据类型：标量类型和复合类型
        pub fn data_types() {
            // Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。
            // 根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，比如使用 parse 将 String 转换为数字时，必须增加类型注解。

            // 标量类型：整型、浮点型、布尔类型和字符类型。

            // 整数类型
            // 有符号数以补码形式存储。
            // 每一个有符号的变体可以储存包含从 -(2的(n - 1)次方) 到 2的(n - 1)次方 - 1 在内的数字，这里 n 是变体使用的位数。所以 i8 可以储存从 -(2的7次方) 到 2的7次方 - 1 在内的数字，也就是从 -128 到 127。
            // 无符号的变体可以储存从 0 到 2的n次方 - 1 的数字，所以 u8 可以储存从 0 到 2的8次方 - 1 的数字，也就是从 0 到 255。
            // 另外，isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的，32 位架构上它们是 32 位的。
            // 多种数字类型的数字字面值允许使用类型后缀，例如 57u8 来指定类型，同时也允许使用 _ 做为分隔符以方便读数，
            // 数字类型默认是 i32。isize 或 usize 主要作为某些集合的索引。
            // 注意：整型溢出
            let a: i8 = 1;
            println!("a = {}", a);
            let b: u16 = 2_000u16;
            println!("b = {}", b);

            // 浮点类型
            // Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。所有的浮点型都是有符号的。
            let f1: f32 = 1.0;
            println!("f1 = {}", f1);
            let f2 = 1.0;
            println!("f2 = {}", f2);

            // 布尔类型
            let d: bool = true;
            println!("d = {}", d);

            // 字符类型
            // 单引号声明 char 字面量，而与之相反的是，使用双引号声明字符串字面量。
            // Rust 的 char 类型的大小为四个字节，并代表了一个 Unicode 标量值（Unicode Scalar Value），这意味着它可以比 ASCII 表示更多内容。
            let c = 'z';
            let z: char = 'ℤ'; // with explicit type annotation
            let heart_eyed_cat = '😻';
            println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);

            // 复合类型: Rust 有两个原生的复合类型：元组（tuple）和数组（array）。
            // 元组类型
            // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定：一旦声明，其长度不会增大或缩小。
            // 我们使用包含在圆括号中的逗号分隔的值列表来创建一个元组。元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。
            // 模式匹配（pattern matching）来解构（destructure）元组值
            // 不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组。这种值以及对应的类型都写作 ()，表示空值或空的返回类型。如果表达式不返回任何其他值，则会隐式返回单元值。
            let tup: (i32, f64, char, bool) = (1, 1.1, '2', true);
            let (x, y, z, b) = tup;
            println!("x = {}, y = {}, z = {},b = {}", x, y, z, b);
            // 也可以使用点号（.）后跟值的索引来直接访问它们。
            println!(
                "tup.0 = {}, tup.1 = {}, tup.2 = {}, tup.3 = {}",
                tup.0, tup.1, tup.2, tup.3
            );

            // 数组类型
            // 数组中的每个元素的类型必须相同。
            // 在栈（stack）而不是在堆（heap）上为数据分配空间，或者是想要确保总是有固定数量的元素时，数组非常有用。
            //  collect 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。当不确定是应该使用数组还是 collect 的时候，那么很可能应该使用 collect。
            // 数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
            // 使用索引来访问数组的元素
            // 注意：索引越界
            let arr: [u32; 5] = [1, 2, 3, 4, 5];
            println!(
                "arr[0] = {}, arr[1] = {}, arr[2] = {}, arr[3] = {}, arr[4] = {}",
                arr[0], arr[1], arr[2], arr[3], arr[4]
            );
            // println!("arr = {:?}", arr);
            // 在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组
            let arr2 = [3; 5];
            println!(
                "arr2[0] = {}, arr2[1] = {}, arr2[2] = {}, arr2[3] = {}, arr2[4] = {}",
                arr2[0], arr2[1], arr2[2], arr2[3], arr2[4]
            );
            // println!("arr2 = {:?}", arr2);
        }

        /// 数值运算
        pub fn number_compute() {
            // addition
            let sum = 5 + 10;
            println!("sum = {}", sum);

            // subtraction
            let difference = 95.5 - 4.3;
            println!("difference = {}", difference);

            // multiplication
            let product = 4 * 30;
            println!("product = {}", product);

            // division
            // 整数除法会向零舍入到最接近的整数。
            let quotient = 56.7 / 32.2;
            let truncated = -5 / 3; // 结果为 -1
            println!("quotient = {}, truncated = {}", quotient, truncated);

            // remainder
            let remainder = 43 % 5;
            println!("remainder = {}", remainder);
        }
    }

    pub mod function {
        /// 函数定义
        pub fn function(a: i32) {
            // fn 关键字，它用来声明新函数。
            // Rust 代码中的函数和变量名使用 snake case 规范风格。在 snake case 中，所有字母都是小写并使用下划线分隔单词。
            // Rust 中通过输入 fn 后面跟着函数名和一对圆括号来定义函数。大括号告诉编译器哪里是函数体的开始和结尾。
            // Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。

            // 参数（parameters）的函数，参数是特殊变量，是函数签名的一部分。
            // 当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。技术上讲，这些具体值被称为参数（arguments）
            // 在函数签名中，必须 声明每个参数的类型。
            // 当定义多个参数时，使用逗号分隔
            println!("a = {}", a);
            // 语句和表达式
            // 函数体由一系列的语句和一个可选的结尾表达式构成。
            // Rust 是一门基于表达式（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。

            // 语句（Statements）是执行一些操作但不返回值的指令。 表达式（Expressions）计算并产生一个值。
            // 语句: 如变量绑定、函数定义也是语句
            let y = 6;
            println!("y = {}", y);
            // let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值。
            // let x = (let y = 6);
            // 表达式：表达式可以是语句的一部分，函数调用是一个表达式。宏调用是一个表达式。用大括号创建的一个新的块作用域也是一个表达式
            // 注意 y+1 这一行在结尾没有分号。表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
            let x = {
                let y = 3;
                y + 1
            };
            println!("x = {}", x);
        }

        /// 返回值
        pub fn return_function(a: i32) -> i32 {
            // 函数返回值
            // 函数可以向调用它的代码返回值。我们并不对返回值命名，但要在箭头（->）后声明它的类型。
            // 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
            // 使用 return 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。
            // 如果在包含 x + 1 的行尾加上一个分号，把它从表达式变成语句，我们将看到一个错误。
            a + 1
        }
    }

    pub mod control_flow {
        /// if 控制
        pub fn control_if() {
            // if 表达式中与条件关联的代码块有时被叫做 arms
            // 代码中的条件 必须 是 bool 值。如果条件不是 bool 值，我们将得到一个错误。
            // 将 else if 表达式与 if 和 else 组合来实现多重条件。
            //  Rust 只会执行第一个条件为 true 的代码块，并且一旦它找到一个以后，甚至都不会检查剩下的条件了。
            let number = 6;
            if number % 4 == 0 {
                println!("number is divisible by 4");
            } else if number % 3 == 0 {
                println!("number is divisible by 3");
            } else if number % 2 == 0 {
                println!("number is divisible by 2");
            } else {
                println!("number is not divisible by 4, 3, or 2");
            }

            // 在 let 语句中使用 if
            // if 是一个表达式，我们可以在 let 语句的右侧使用它
            // if 的每个分支的可能的返回值都必须是相同类型
            let condition = true;
            let number = if condition { 5 } else { 6 };
            // let number = if condition { 5 } else { "six" };
            println!("The value of number is: {number}");
        }

        /// 循环
        pub fn control_loops() {
            // 循环
            // Rust 有三种类型的循环：loop、while 和 for。
            //  循环中的 continue 关键字告诉程序跳过这个循环迭代中的任何剩余代码，并转到下一个迭代。
            println!("loop start:-------------------");
            control_loop();
            println!("loop end:-------------------");
            println!("while start:-------------------");
            control_while();
            println!("while end:-------------------");
            println!("for start:-------------------");
            control_for();
            println!("for end:-------------------");
        }

        /// loop 循环
        pub fn control_loop() {
            // loop 循环
            // loop 循环会一直运行下去，直到被显式地结束。
            // 从循环返回值
            // 使用 break 关键字返回值 counter * 2。循环之后，我们通过分号结束赋值给 result 的语句。
            let mut counter = 0;
            let result = loop {
                if counter == 5 {
                    break counter * 2;
                }
                counter += 1
            };
            println!("result = {}", result);

            // 循环标签：在多个循环之间消除歧义
            // 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
            // 你可以选择在一个循环上指定一个 循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环。

            let mut count = 0;
            'loop1: loop {
                println!("count = {}", count);
                let mut inner_count = 10;
                'loop2: loop {
                    println!("inner_count = {}", inner_count);
                    if inner_count == 9 {
                        break;
                    }
                    if count == 2 {
                        break 'loop1;
                    }
                    inner_count -= 1;
                }
                count += 1;
            }
        }

        /// while 循环
        pub fn control_while() {
            // while 循环
            // while 循环会一直运行下去，直到条件为 false。
            // while 循环和 loop 循环一样，都有返回值。
            // while 循环和 loop 循环一样，都有 continue 关键字。
            let mut number = 3;
            while number != 0 {
                println!("{number}!");
                number -= 1;
            }
            println!("LIFTOFF!!!");
        }

        /// for 循环
        pub fn control_for() {
            let arr = [2; 5];
            for ele in arr {
                println!("ele = {}", ele);
            }
            println!("-----------------");
            for ele in arr.iter() {
                println!("ele = {}", ele);
            }
            println!("-----------------");

            //  Range，它是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。
            // rev，用来反转 range。
            for num in (1..4).rev() {
                println!("{}!", num);
            }
            println!("LIFTOFF!!!");
        }
    }
}
