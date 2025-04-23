// main.rs
use minimal_async_runtime::{MiniRuntime, spawn, sleep};
use std::time::Duration;

async fn task_one() {
    println!("task one: start");
    sleep(Duration::from_secs(1)).await;
    println!("task one: done   [~1s]");
}

async fn task_two() {
    println!("task two: start");
    sleep(Duration::from_secs(2)).await;
    println!("task two: done   [~2s]");
}

fn main() {
    let mut runtime = MiniRuntime::new();
    runtime.block_on(async {
        println!("Runtime started...");
        
        let handle1 = spawn(task_one());
        let handle2 = spawn(task_two());
        
        handle1.await;
        handle2.await;
    });
}