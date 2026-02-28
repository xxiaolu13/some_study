use std::{io::read_to_string, pin::Pin, process::Output, thread::yield_now};

// async await future
struct Response;
struct Request;
trait Service{
    // fn call(&mut self,_ : Request) -> impl Future<Output = Response>{}
    fn call(&mut self,_ : Request) -> Pin<Box<dyn Future<Output = Response>>>{}
}
async fn fooo(x: &mut dyn Service) {
    let fut = x.call(Request);
}
// 实际上 futrue状态机 会生成一个类似枚举的东西，然后它内部保存了future的每个检查点，像x这种都存储在heap上，使用指针即可，减少内存复制的次数
// 结合tokio spawn 他不必存储future，只是存状态机的指针并使用，
enum Statemachine {
    chunk1 {x: &[u8],tokio::fs::ReadIntoFuture<'x>},
    chunk2 {},
}
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // 感受下future -> 状态机
        // chunk1
        {
            let mut x = &[0, 1024];
            let y = tokio::rs::read_into("t", &mut x[..]);
        }
        // yield -> really: return
        // chunk2
        {
            let n = y.output();
            println!("{:?}", n);
        }

        // select 的选择机制
        // biased 顺序选择
        // 开始轮询所有分支
        //     ↓
        // foo1 开始跑，遇到 read_to_string("test_1").await 挂起
        //     ↓
        // 此时 network 好了 → select! 选择 network 分支
        //     ↓
        // foo1 被直接丢弃，永远不会继续执行
        let x = select! {
            stream <- network.await =>{
                // process the stream
            }
            line <- terminal.await => {
                // process the line
            }
            _ = foo1() => {
                // process the foo1
            }
        };
        // join 相关两种用法 join all 当然也有try join all ，还有join！
        // join会保证输出的顺序 只检查发生过的
        let join_handle = tokio::spawn(async move {
            let x: Result<_, _> = definitenly_err();
            // tokio spqwn 中处理错误 比较难受可以记录到日志里面，相对优雅了
        });
        let files = (0..5)
            .map(|i| tokio::fs::read_to_string(format!("test_{}", i)).await)
            .collect::<Vec<_>>();
        let list = join_all(files);
        let a = files[0].await;
        let b = files[1].await;
        let (a, b) = join!(a, b);
    });
}
async fn foo1() {
    println!("foo1");
    tokio::fs::read_to_string("test_1").await;
    println!("foo2");
    tokio::fs::read_to_string("test_2").await;
    println!("foo3");
    tokio::fs::read_to_string("test_3").await;
    println!("foo4");
    tokio::fs::read_to_string("test_4").await;
    println!("foo5");
    tokio::fs::read_to_string("test_5").await;
}
fn foo2() -> impl Future<Output = ()> {
    async {
        println!("foo2");
        // desugur1
        // let x = read_to_string("test_file").await;
        // let fut = read_to_string("test_file").await;
        // while !fut.is_read() {
        //     yield_now();
        //     fut.try_complete();
        // }

        // desugur2
        // loop {
        //     if let Some(e) = try_check_com("test_file") {
        //         break e;
        //     } else {
        //         //     fut.try_make_progress();
        //         // //     yield_now();
        //     }
        // }
    }
}
