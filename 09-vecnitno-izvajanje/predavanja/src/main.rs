use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};
use std::sync::mpsc;

fn print_pro_tip(i: i32) {
    // let delay = rand::thread_rng().gen_range(1000..=5000);
    thread::sleep(time::Duration::from_millis(delay));
    let n = 42 - i;
    println!("Pro tip #{i}: {n} + {i} = 42");
}

fn send_pro_tip(i: i32, tx: mpsc::Sender<(i32, String)>) {
    // let delay = rand::thread_rng().gen_range(1000..=5000);
    // thread::sleep(time::Duration::from_millis(delay));
    let n = 42 - i;
    tx.send((i, format!("Pro tip #{i}: {n} + {i} = 42")));
}

fn read_pro_tips(rx: mpsc::Receiver<(i32, String)>) {
    let mut pro_tips = HashMap::new();
    let mut i = 1;
    for (n, pro_tip) in rx {
        pro_tips.insert(n, pro_tip);
        while let Some(pro_tip) = pro_tips.get(&i) {
            i = i + 1;
            println!("{pro_tip}");
        }
    }
}

fn main() {
    for i in 1..5000 {
        print_pro_tip(i);
    }

    // let mut handles = vec![];
    // for i in 1..10000 {
    //     println!("{i}");
    //     let handle = thread::spawn(move || print_pro_tip(i));
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join();
    // }

    // let mut handles = vec![];
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(|| read_pro_tips(rx));
    // for i in 1..5000 {
    //     let tx_clone = tx.clone();
    //     let handle = thread::spawn(move || send_pro_tip(i, tx_clone));
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join();
    // }
}
