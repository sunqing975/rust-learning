/// vector
pub fn vector_def() {
    // 创建一个新的空 vector，可以调用 Vec::new 函数
    let mut v = Vec::new();
    // Rust 提供了 vec! 宏，这个宏会根据我们提供的值来创建一个新的 vector。
    let mut v2 = vec![1, 2, 3];

    let mut v3 = vec!["1", "2", "3"];

    let mut v4 = vec![String::from("1"), String::from("2"), String::from("3")];

    // 更新vector ，push 方法。更改为可变变量
    v.push(5);
    v2.push(5);

    // 读取
    // 使用v[0]和&v[0]的区别：（v[index]返回的是对应元素的引用）
    //     当v存储的是整型、浮点数等实现了Copy trait接口的类型时，v[0]会返回一个值（自动解引用），而&v[0]会返回一个引用，但是在后续使用的时候也会自动解引用。
    //     当v存储的是切片（字符串字面量）时，对应的类型是引用，其没有实现Copy trait接口的类型时，v[0]会返回一个引用，而&v[0]会返回一个引用的引用，但是在后续使用的时候会自动解引用。
    //     当v存储的是字符串时，对应的类型是字符串，其没有实现Copy trait接口的类型时，v[0]会返回一个字符串(但是程序无法编译)，而&v[0]会返回一个字符串的引用，但是在后续使用的时候会自动解引用。
    let m1 = v[0];
    let m1 = &v[0];
    // 当上一行代码注释后，编译器可以正常运行，
    // v.push(6);
    println!("The first element is: {}", m1);
    println!("The first element is: {:?}", v);

    let m2 = v2[1];
    let m2 = &v2[1];
    println!("The second element is: {}", m2);
    println!("The second element is: {:?}", v2);

    let m3 = v3[2];
    let m3 = &v3[2];
    // 当上一行代码注释后，编译器可以正常运行，
    // v3.push("4");
    println!("The third element is: {}", m3);
    println!("The third element is: {:?}", v3);


    // let m4 = v4[1];
    // 不能在相同作用域中同时存在可变和不可变引用
    let m4 = &v4[1];
    // mutable borrow occurs here
    // v4.push(String::from("4"));
    println!("The third element is: {}", m4);

    // 这里如果不使用引用的话，v4的所有权会丢失，看情况使用引用还是使用值
    // for x in v4 {
    for x in &v4 {
        println!("{}", x)
    }

    // 因为借用检查器的规则，无论可变还是不可变地遍历一个 vector 都是安全的。
    for x in &mut v2 {
        // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
        *x += 100;
    }
    println!("{:?}", v2)
}


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


pub fn vector_enum() {
    // 如果在编写程序时不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了。相反，你可以使用 trait 对象
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.1),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}