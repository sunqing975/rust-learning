use trpl::{ReceiverStream, Stream, StreamExt};

pub fn stream_demo() {
    // 流类似于一种异步形式的迭代器。
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }
    // 将 trpl::channel 的 rx 接收端转换为一个带有带有 next 方法的 Stream。
    ReceiverStream::new(rx)
}

pub fn stream_demo2() {
    trpl::run(async {
        let mut messages = get_messages();
        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });
}
