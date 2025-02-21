/*
    猜数游戏
*/
use std::cmp::Ordering;
use std::io;
use rand::Rng;

/// 猜数游戏主函数
pub fn guess_number() {
    // 游戏开始提示信息
    println!("猜数游戏");
    // 生成1到100之间的随机数作为秘密数字
    let secret_number = rand::thread_rng().gen_range(1..=101);

    // 主循环，处理用户的猜测
    loop {
        // 提示用户输入猜测
        println!("请输入你的猜测：");
        // 创建一个可变的字符串来存储用户的输入
        let mut guess = String::new();
        // 读取用户的输入，处理读取失败的情况
        io::stdin().read_line(&mut guess).expect("读取失败");
        // 将用户的输入转换为无符号整数，如果转换失败则继续下一次循环
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 比较用户的猜测和秘密数字，给出相应的提示
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                // 如果猜对了，结束循环
                println!("猜对了");
                break;
            }
        }
    }
}
