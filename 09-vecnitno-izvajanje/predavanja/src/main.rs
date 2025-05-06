use rand::Rng;
use std::{io, thread, time};

fn print_pro_tip(i: i32) {
    let delay = rand::thread_rng().gen_range(1000..=5000);
    thread::sleep(time::Duration::from_millis(delay));
    let n1 = rand::thread_rng().gen_range(0..=100);
    let n2 = 42 - n1;
    println!("Pro tip #{i}: {n2} + {n1} = 42")
}

fn main() {
    // for i in 1..100 {
    //     print_pro_tip(i);
    // }

    let mut niti = vec![];
    for i in 1..100 {
        let nit = thread::spawn(move || print_pro_tip(i));
        niti.push(nit);
    }
    for nit in niti {
        nit.join();
    }
}
