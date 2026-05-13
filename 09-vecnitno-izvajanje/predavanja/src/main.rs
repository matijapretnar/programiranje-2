use rand::Rng;
use std::sync::mpsc;
use std::{thread, time};

fn print_pro_tip(i: i32) {
    let delay = rand::thread_rng().gen_range(1000..=5000);
    thread::sleep(time::Duration::from_millis(delay));
    let n = 42 - i;
    println!("Pro tip #{i}: {n} + {i} = 42 ({delay}ms)");
}

fn pocasno_zaporedno_izvajanje() {
    for i in 1..5000 {
        print_pro_tip(i);
    }
}

fn vzporedne_niti_ki_jih_prehitro_opustimo() {
    for i in 1..5000 {
        thread::spawn(move || print_pro_tip(i));
    }
    println!("Malo bom še počakal");
    thread::sleep(time::Duration::from_millis(500));
    println!("Pa še malo");
    thread::sleep(time::Duration::from_millis(100));
    println!("Zdaj pa bom končal");
}

fn vzporedne_niti_ki_jih_sproti_cakamo() {
    for i in 1..5000 {
        let h = thread::spawn(move || print_pro_tip(i));
        h.join();
    }
}

fn vzporedne_niti_ki_ustrezno_pocakamo() {
    let mut hs = vec![];
    for i in 1..5000 {
        let h = thread::spawn(move || print_pro_tip(i));
        hs.push((i, h));
    }
    for (i, h) in hs {
        println!("Čakam na pro tip #{i}");
        h.join();
    }
}

fn send_pro_tip(i: i32, tx: mpsc::Sender<String>) {
    let delay = rand::thread_rng().gen_range(1000..=5000);
    thread::sleep(time::Duration::from_millis(delay));
    let n = 42 - i;
    tx.send(format!("Pro tip #{i}: {n} + {i} = 42 ({delay}ms)"));
}

fn vzporedne_niti_ki_komunicirajo_s_klicateljem() {
    let mut hs = vec![];
    let (tx, rx) = mpsc::channel();
    for i in 1..10000 {
        let tx_clone = tx.clone();
        println!("Poganjam {i}-to nit");
        let h = thread::spawn(move || send_pro_tip(i, tx_clone));
        hs.push(h);
    }
    for m in rx {
        println!("{m}");
    }
}

fn main() {
    // pocasno_zaporedno_izvajanje();
    // vzporedne_niti_ki_jih_prehitro_opustimo();
    // vzporedne_niti_ki_jih_sproti_cakamo();
    // vzporedne_niti_ki_ustrezno_pocakamo();
    vzporedne_niti_ki_komunicirajo_s_klicateljem();
}
