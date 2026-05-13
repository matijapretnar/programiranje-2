fn a() {
    println!("Začenjam A");
    for i in 1..=5 {
        println!("A {i}");
    }
    println!("Končujem A");
}

fn b() {
    for i in 1..=5 {
        println!("B {i}");
    }
    println!("B prehaja na drugo polovico");
    for i in 6..=10 {
        println!("B {i}");
    }
    println!("Končujem B");
}

fn a_in_b_zaporedno() {
    println!("A in B bom pognal zaporedno");
    a();
    b();
}

enum StateA {
    Start,
    BeforePrint(i32),
    Done,
}
enum StateB {
    Start,
    FirstHalf(i32),
    SecondHalf(i32),
    Done,
}

// fn a() {
//     println!("Začenjam A");
//     for i in 1..=5 {
//         println!("A {i}");
//     }
//     println!("Končujem A");
// }

fn step_a(state: StateA) -> StateA {
    match state {
        StateA::Start => {
            println!("Začenjam A");
            StateA::BeforePrint(1)
        }
        StateA::BeforePrint(i) => {
            println!("A {i}");
            if i < 5 {
                StateA::BeforePrint(i + 1)
            } else {
                println!("Končujem A");
                StateA::Done
            }
        }
        StateA::Done => StateA::Done,
    }
}

fn stari_a() {
    let mut state = StateA::Start;
    loop {
        state = step_a(state);
        if let StateA::Done = state {
            break;
        }
    }
}

fn step_b(state: StateB) -> StateB {
    match state {
        StateB::Start => StateB::FirstHalf(1),
        StateB::FirstHalf(i) => {
            println!("B {i}");
            if i < 5 {
                StateB::FirstHalf(i + 1)
            } else {
                println!("B prehaja na drugo polovico");
                StateB::SecondHalf(6)
            }
        }
        StateB::SecondHalf(i) => {
            println!("B {i}");
            if i < 10 {
                StateB::SecondHalf(i + 1)
            } else {
                println!("Končujem B");
                StateB::Done
            }
        }
        StateB::Done => StateB::Done,
    }
}

enum Task {
    A(StateA),
    B(StateB),
}

fn run(mut queue: Vec<Task>) {
    while !queue.is_empty() {
        let task = queue.remove(0);
        match task {
            Task::A(StateA::Done) | Task::B(StateB::Done) => {}
            Task::A(s) => queue.push(Task::A(step_a(s))),
            Task::B(s) => queue.push(Task::B(step_b(s))),
        }
    }
}

fn a_in_b_izmenicno() {
    println!("A in B bom pognal izmenično");
    run(vec![Task::A(StateA::Start), Task::B(StateB::Start)]);
}

async fn dodaj_klicaj(str: String) -> String {
    format!("{str}!")
}

async fn izpisi_in_pocakaj(str : String) {
    let niz_s_klicajem = dodaj_klicaj(str).await;
    println!("{niz_s_klicajem}");
    // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
}

async fn asinh_a() {
    let a = izpisi_in_pocakaj("Začenjam A".into());
    for i in 1..=5 {
            izpisi_in_pocakaj(format!("A {i}")).await;
    }
    izpisi_in_pocakaj("Končujem A".into()).await;
}

async fn asinh_b() {
    for i in 1..=5 {
            izpisi_in_pocakaj(format!("B {i}")).await;
    }
    izpisi_in_pocakaj("B prehaja na drugo polovico".into()).await;
    for i in 6..=10 {
            izpisi_in_pocakaj(format!("B {i}")).await;
    }
    izpisi_in_pocakaj("Končujem B".into()).await;
}

async fn a_in_b_asinhrono() {
    println!("A in B bom pognal asinhrono");
    let ha = tokio::spawn(asinh_a());
    let hb = tokio::spawn(asinh_b());
    tokio::join!(ha, hb);
}

#[tokio::main]
async fn main() {
    // a_in_b_zaporedno();
    // a_in_b_izmenicno();
    a_in_b_asinhrono().await;
}
