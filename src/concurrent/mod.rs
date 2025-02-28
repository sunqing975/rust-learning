pub mod stream;

use std::future::Future;
use std::pin::{pin, Pin};
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use trpl::{Either, Html};

/// 并发编程
pub fn concur_demo() {
    // 并发编程（Concurrent programming），代表程序的不同部分相互独立地执行，
    // 并行编程（parallel programming）代表程序不同部分同时执行，这两个概念随着计算机越来越多的利用多处理器的优势而显得愈发重要。

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(100));
        }
    });
    // 使用 join 等待所有线程结束
    // 将 handle.join() 移动到 main 中 for 循环之前
    // 主线程会等待直到新建线程执行完毕之后才开始执行 for 循环，所以输出将不会交替出现
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(100));
    }
    // 使用 join 等待所有线程结束
    // 调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。阻塞（Blocking）线程意味着阻止该线程执行工作或退出。
    handle.join().unwrap();
    println!("------------------------");
    move_thread();
    println!("------------------------");
    thread_channel();
    println!("------------------------");
    thread_channel_multiple();
}

fn move_thread() {
    let v = vec![1, 2, 3];
    // closure may outlive the current function, but it borrows `v`, which is owned by the current function : 闭包的寿命可能比当前函数长，但它借用了当前函数所拥有的v
    // let handle = thread::spawn(|| {
    // 闭包之前增加 move 关键字，我们强制闭包获取其使用的值的所有权，而不是任由 Rust 推断它应该借用值。
    let handle = thread::spawn(move || println!("{:?}", v));

    // value used here after move
    // drop(v);

    handle.join().unwrap();
}

fn thread_channel() {
    // Rust 标准库提供了一个 信道（channel）实现。信道是一个通用编程概念，表示数据从一个线程发送到另一个线程。
    // 编程中的信息渠道（信道）有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。
    // 当发送者或接收者任一被丢弃时可以认为信道被 关闭（closed）了。
    // 使用 mpsc::channel 函数创建一个新的信道；mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // value borrowed here after move
        // println!("val is {val}");
    });
    // 信道的接收者有两个有用的方法：recv 和 try_recv。
    // 当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
    // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
    // 如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn thread_channel_multiple() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 在主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当信道被关闭时，迭代器也将结束。
    for received in rx {
        println!("Got: {received}");
    }
}

/// 共享内存
pub fn sharing_memory_demo() {
    // 共享内存类似于多所有权：多个线程可以同时访问相同的内存位置。
    // 互斥器（mutex）是 互相排斥（mutual exclusion）的缩写。
    // 在同一时刻，其只允许一个线程对数据拥有访问权。为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。锁是一个数据结构，作为互斥器的一部分，它记录谁有数据的专属访问权。因此我们讲，互斥器通过锁系统 保护（guarding）其数据。

    // 在使用数据之前，必须获取锁。
    // 使用完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

    let m = Mutex::new(5);

    {
        // 一旦获取了锁，就可以将返回值（命名为 num）视为一个其内部数据（i32）的可变引用了。
        // 类型系统确保了我们在使用 m 中的值之前获取锁。m 的类型是 Mutex<i32> 而不是 i32，所以 必须 获取锁才能使用这个 i32 值。
        // 我们是不会忘记这么做的，因为如果没有获取锁，类型系统就不允许访问内部的 i32 值。
        // lock 调用 返回 一个叫做 MutexGuard 的智能指针。这个智能指针实现了 Deref 来指向其内部数据；它也实现了 Drop，当 MutexGuard 离开作用域时，自动释放锁（发生在示例 16-12 内部作用域的结尾）。
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    println!("------------------------");
    thread_mutex();
}

fn thread_mutex() {
    // 互斥器（mutex）是 互相排斥（mutual exclusion）的缩写。
    //  `Rc<Mutex<i32>>` cannot be sent between threads safely
    // the trait `Send` is not implemented for `Rc<Mutex<i32>>`

    // Rc<T> 并没有使用任何并发原语，无法确保改变计数的操作不会被其他线程打断。这可能使计数出错，并导致诡异的 bug，
    // 比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。我们所需要的是一个与 Rc<T> 完全一致，又以线程安全的方式改变引用计数的类型。
    // let counter = Rc::new(Mutex::new(0));
    //  Arc<T> 正是这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        // let counter = Rc::clone(&counter);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    // RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
    // 尽管 counter 是不可变的，我们仍然可以获取其内部值的可变引用；这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样。
    //  死锁（deadlock）的风险：当某个操作需要锁住两个资源，而两个线程分别持有两个资源的其中一个锁时，它们会永远相互等待。
}

/// 线程间的通信
pub fn sync_send_demo() {
    // Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送。
    // 例外，包括 Rc<T>：这是不能 Send 的，因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权转移到另一个线程，这两个线程都可能同时更新引用计数。为此，Rc<T> 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价。
    // 任何完全由 Send 的类型组成的类型也会自动被标记为 Send。几乎所有基本类型都是 Send 的，除了第二十章将会讨论的裸指针（raw pointer）。

    // Sync 允许多线程访问
    // Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
    // 对于任意类型 T，如果 &T（T 的不可变引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程。类似于 Send 的情况，基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。

    // 手动实现 Send 和 Sync 是不安全的
}

pub fn async_await() {
    //  注意：视频导出这类操作通常被称为 “CPU 密集型”（“CPU-bound”）或者 “计算密集型”（“compute-bound”）操作。
    // 其受限于计算机 CPU 或 GPU 处理数据的速度，以及它所能利用的计算能力。而下载视频这类操作通常被称为 “IO 密集型”（“IO-bound”）操作，因为其受限于计算机的 输入输出 速度。
    // 下载的速度最多只能与通过网络传输数据的速度一致。

    // 在 Rust 中，我们称实现了 Future trait 的类型为 futures。每一个实现了 Future 的类型会维护自己的进度状态信息和 “ready” 的定义。
    // async 关键字可以用于代码块和函数，表明它们可以被中断并恢复。
    // 在一个 async 块或 async 函数中，可以使用 await 关键字来等待一个 future 准备就绪，这一过程称为 等待一个 future。
    // async 块或 async 函数中每一个等待 future 的地方都可能是一个 async 块或 async 函数中断并随后恢复的点。检查一个 future 并查看其值是否已经准备就绪的过程被称为 轮询（polling）。
}

pub fn hello_async() {
    let args: Vec<String> = std::env::args().collect();
    //  trpl crate 的 run 函数，它获取一个 future 作为参数并运行到结束。在内部，调用 run 会设置一个运行时来运行传递的 future。一旦 future 完成，run 返回 future 返回的任何值。
    // trpl::run(async {
    //     let url = &args[1];
    //     match page_title(url).await {
    //         Some(title) => println!("The title for {url} was {title}"),
    //         None => println!("{url} had no title"),
    //     }
    // });

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        // futures 传递给 trpl::race，它返回一个值表明哪个传递的 future 最先返回。
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            // 由于任何一个 future 都可以合理地 “获胜”，所以返回 Result 没有意义。相反 race 返回了一个我们之前没有见过的类型 trpl::Either。
            // Either 类型有点类似于 Result，它也有两个成员。但是不同于 Either，Either 没有内置成功或者失败的概念。相反它使用 Left 和 Right 来表示 “一个或另一个”。
            // race 函数返回 Left，如果第一个参数先完成，并包含该 future 的输出，
            // 如果 第二个 future 先完成，则返回 Right 和第二个 future 的输出。这匹配调用函数时参数出现的顺序：第一个参数在第二个参数的左边。
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// async fn page_title(url: &str) -> Option<String> {
//     let response = trpl::get(url).await;
//     let response_text = response.text().await;
//     // let response_text = trpl::get(url).await.text().await;
//
//     Html::parse(&response_text)
//         .select_first("title")
//         .map(|title_element| title_element.inner_html())
// }

// 与page_title等价
// 新版本的函数在返回类型中使用了一种我们之前未见过的生命周期标记：'_。
// 因为函数返回的 Future 指向一个引用（在这个例子中是指向 url 参数的引用）我们需要告诉 Rust 引用的生命周期。
// 这里无需命名该生命周期，因为 Rust 足够智能到能理解这里只涉及到唯一一个引用，不过我们 必须 明确指出返回的 Future 受该生命周期的约束。
fn page_title_2(url: &str) -> impl Future<Output = Option<String>> + '_ {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}

pub fn async_await_2() {
    trpl::run(async {
        // rpl crate 提供了一个 spawn_task 函数，它看起来非常像 thread::spawn API，和一个 sleep 函数，这是 thread::sleep API 的异步版本。
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // 对于线程来说，可以使用 join 方法来 “阻塞” 直到线程结束运行。
        // 我们可以使用 await 来实现相同的效果，因为任务句柄本身是一个 future。它的 Output 类型是一个 Result，所以我们还需要 unwrap 来 await 它。
        // 异步代码块会编译为匿名 future，我们可以将每一个循环放进一个异步代码块并使用 trpl::join 方法来让运行时将它们两个都运行至完成。
        handle.await.unwrap();
    });
}

pub fn async_await_3() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // 这里，你每次都会看到完全相同的顺序，这与我们在线程中看到的情况非常不同。这是因为 trpl::join 函数是 公平的（fair），
        // 这意味着它以相同的频率检查每一个 future，使它们交替执行，绝不会让一个任务在另一个任务准备好时抢先执行。
        // 对于线程来说，操作系统会决定该检查哪个线程和会让它运行多长时间。对于异步 Rust 来说，运行时决定检查哪一个任务。
        trpl::join(fut1, fut2).await;
    });
}

pub fn async_await_4() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    });
}

pub fn async_await_5() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
        // while let 循环是我们在第六章中见过的 if let 结构的循环版本。只要其指定的模式持续匹配循环就会一直执行。
        // rx.recv 调用产生一个 Future，我们会 await 它。运行时会暂停 Future 直到它就绪。一旦消息到达，future 会解析为 Some(message)，
        // 每次消息到达时都会如此。。当信道关闭时，不管是否有 任何 消息到达，future 都会解析为 None 来表明没有更多的值了，我们也就应该停止轮询，也就是停止等待。
        // 它们在程序启动后两秒（2000 毫秒）后立刻一起到达。
        // 只有一个异步代码块，所以所有的代码线性地执行。这里仍然没有并发。
        // 所有 tx.send 调用与 trpl::sleep 调用及其相关的 await point 是依次进行的。
        // 只有在此之后 while let 循环才开始执行 recv 调用上的 await point。
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}

pub fn async_await_6() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();
        // 不加move时，tx的所有权没有传递给rx_fut，所以程序无法结束
        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                // 目前发送消息的异步代码块只是借用了 tx，因为发送消息并不需要其所有权
                //  move 关键字也能像闭包那样作用于异步代码块。
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        // trpl::join(tx_fut, rx_fut).await;
        // 两个 future 到三个 future 的时候，我们也必须从使用 join 切换到 join3
        // trpl::join3(tx_fut, rx_fut, rx_fut).await;
        // 宏版本的 join 可以传递任意数量的参数。它还会自行处理 await 这些 future。
        // 即便是这个宏形式也只能用于我们提前知道 future 的数量的情况。
        // trpl::join!(tx_fut, rx_fut);

        // trpl::join_all 函数接受任何实现了 Iterator trait 的类型
        // 使用 trait objects 允许我们将这些类型所产生的不同的匿名 future 视为相同的类型，因为它们都实现了 Future trait。
        // expected `async` block, found a different `async` block
        // trpl::join_all(vec![tx_fut, rx_fut]).await;
        // the trait `Unpin` is not implemented for `dyn Future<Output = ()>`
        // 如果需要在当前范围之外访问固定值，请考虑使用`box :: pin` or `pin!` macro
        // let futures: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(tx_fut), Box::new(rx_fut)];

        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        //     vec![Box::pin(rx_fut), Box::pin(tx_fut)];
        // 在 Rust 中，Pin（全称为 Pin<P>）是一个封装类型，用于确保某些对象在其生命周期内不会被移动。
        // 这对于一些需要保持其内存地址不变的场景特别有用，例如自引用结构体和异步编程中的协程。
        // 它的主要作用是提供一种机制来“钉住”（pin）某个指针类型的对象，防止它在内存中被移动。
        // 具体来说，Pin<P> 确保了指向的对象在其生命周期内不会被移动到其他内存位置。
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![rx_fut, tx_fut];

        trpl::join_all(futures).await;
    })
}

pub fn async_await_7() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };
        // futures 传递给 trpl::race，它返回一个值表明哪个传递的 future 最先返回。
        // trpl::race(slow, fast).await;
        // 这个特定的 race 函数实现并不是公平的。它总是以传递的参数的顺序来运行传递的 futures。其它的实现 是 公平的，并且会随机选择首先轮询的 future。
        // 不过无论我们使用的 race 实现是否公平，其中 一个 future 会在另一个任务开始之前一直运行到异步代码块中第一个 await 为止。
        // 如果被 await 的 future 还没有就绪，Rust 会给运行时一个机会来暂停该任务并切换到另一个任务。
        // 反过来也是正确的：Rust 只会 在一个 await point 暂停异步代码块并将控制权交还给运行时。await points 之间的一切都是同步。
        // 这意味着如果你在异步代码块中做了一堆工作而没有一个 await point，则那个 future 会阻塞其它任何 future 继续进行。
        // 不过，如果你在进行某种昂贵的设置或者长时间运行的任务，亦或有一个 future 会无限持续运行某些特定任务的话，你会需要思考在何时何地将控制权交还运行时。
        trpl::race(fast, slow).await;
    })
}

pub fn async_await_yielding() {
    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
        // 由于异步任务通常运行在一个线程池中的某个线程上，即使任务让出了控制权，该线程仍然被占用。
        // 也就是说，线程本身并没有被释放，只是当前任务暂时停止执行，调度器可以选择其他任务在这个线程上执行。

        // 在多线程环境中，线程是操作系统级别的资源。当一个线程调用类似 std::thread::yield_now() 的方法时，
        // 它会让出当前线程的时间片，使得操作系统可以调度其他线程来执行。但是，这个线程仍然处于可执行状态，并且可能会很快再次被调度执行。
        // trpl::race(a, b).await;
    })
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

pub fn async_await_timeout() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
