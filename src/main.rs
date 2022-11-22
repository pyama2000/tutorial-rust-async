#[tokio::main]
async fn main() {
    for _ in 0..=5 {
        let _ = tokio::join!(run_concurrency(), run_parallelism());
    }
}

/// 並列にタスクを動かす関数
/// future1, future2, future3が順番に実行される
async fn run_concurrency() {
    let future1 = sleep("concurrency 1");
    let future2 = sleep("concurrency 2");
    let future3 = sleep("concurrency 3");
    let _ = tokio::join!(future1, future2, future3);
}

/// 並行にタスクを動かす関数
/// future1, future2, future3が順不同で実行される
async fn run_parallelism() {
    let future1 = sleep("parallelism 1");
    let future2 = sleep("parallelism 2");
    let future3 = sleep("parallelism 3");
    let handler1 = tokio::spawn(future1);
    let handler2 = tokio::spawn(future2);
    let handler3 = tokio::spawn(future3);
    let _ = tokio::join!(handler1, handler2, handler3);
}

/// 5秒間待つ関数
/// 関数の最初と5秒間待った後に `name` を出力する
async fn sleep(name: &str) {
    println!("start {}", name);
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    println!("finish {}", name);
}
