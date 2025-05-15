use std::time;
use rand::Rng;
use tokio;

async fn print_pro_tip(i: i32) -> i32 {
    let delay = rand::thread_rng().gen_range(1000..=5000);
    let n = 42 - i;
    tokio::time::sleep(time::Duration::from_millis(delay)).await;
    println!("Pro tip #{i}: {n} + {i} = 42");
    20
}

#[tokio::main]
async fn main() {
    // for i in 1..100 {
    //     print_pro_tip(i);
    // }

    let mut handles = vec![];
    let f = print_pro_tip(10);
    let g = f.await + 20;
    for i in 1..50000 {
        let handle = tokio::spawn(print_pro_tip(i));
        handles.push(handle);
    }
    for handle in handles {
        handle.await;
    }
}
