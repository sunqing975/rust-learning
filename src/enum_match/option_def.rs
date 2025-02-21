/// 定义一个函数来演示如何使用Option类型
pub fn option_use() {
    // 初始化一个具有Some值的Option类型变量
    let num = Some(5);
    // 打印显示num的值
    println!("num is {:?}", num);

    // 初始化一个具有None值的Option类型变量，显式指定类型为Option<i32>
    let none: Option<i32> = None;
    // 打印显示none的值
    println!("none is {:?}", none);

    // 初始化一个具有Some值的Option类型变量
    let opt = Some('1');
    // 打印显示opt的值
    println!("opt is {:?}", opt);

    // 初始化一个i32类型的变量
    let some_u8_value = 0i32;
    // 以下代码行将产生编译错误，因为不能直接对Option<i32>和i32进行加法操作
    // 编译器不允许像一个肯定有效的值那样使用 Option<T>。
    // let res = some_u8_value + num;
}