use std::collections::HashMap;

/// 哈希 map
pub fn hashmap_def() {
    // HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
    // 它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。
    // 哈希 map 可以用于需要任何类型作为键来寻找数据的情况，而不是像 vector 那样通过索引。
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // 程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
    // 接着调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零。
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("{:?}", score),
        None => println!("None"),
    }

    // 如果使用scores，则使用权被移动
    for (k, v) in &scores {
        println!("{:?},{:?}", k, v)
    }

    // 对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
    // 对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // println!("{:?}", field_name);

    // 如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。
    // 但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。

    //更新
    let mut scores = HashMap::new();
    // 覆盖一个值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");

    // 只在键没有对应值时插入键值对
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 哈希 map 有一个特有的 API，叫做 entry，它获取我们想要检查的键作为参数。
    // entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值。
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    // 根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert 方法返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}