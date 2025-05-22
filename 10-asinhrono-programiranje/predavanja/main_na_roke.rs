use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use rand::Rng;
use tokio::task::JoinHandle;

//
// Manual Future for print_pro_tip
//

pub struct PrintProTip {
    state: State,
    i: i32,
    delay: u64,
    sleep: Option<Pin<Box<tokio::time::Sleep>>>,
}

enum State {
    Init,
    Sleeping,
    Done,
}

impl PrintProTip {
    pub fn new(i: i32) -> Self {
        let delay = rand::thread_rng().gen_range(1000..=5000);
        Self {
            state: State::Init,
            i,
            delay,
            sleep: None,
        }
    }
}

impl Future for PrintProTip {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::Init => {
                    let sleep = Box::pin(tokio::time::sleep(Duration::from_millis(self.delay)));
                    self.sleep = Some(sleep);
                    self.state = State::Sleeping;
                }
                State::Sleeping => {
                    let sleep = self.sleep.as_mut().unwrap();
                    match sleep.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(()) => {
                            let n = 42 - self.i;
                            println!("Pro tip #{}: {} + {} = 42", self.i, n, self.i);
                            self.state = State::Done;
                        }
                    }
                }
                State::Done => return Poll::Ready(20),
            }
        }
    }
}

//
// Manual Future for main()
//

enum MainState {
    AwaitFirst(Pin<Box<dyn Future<Output = i32> + Send>>),
    SpawnLoop(i32, Vec<JoinHandle<i32>>, i32),
    AwaitAll(usize, Vec<JoinHandle<i32>>),
    Done,
}

pub struct MainFuture {
    state: MainState,
}

impl MainFuture {
    pub fn new() -> Self {
        let f = Box::pin(PrintProTip::new(10));
        Self {
            state: MainState::AwaitFirst(f),
        }
    }
}

impl Future for MainFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut self.state {
                MainState::AwaitFirst(fut) => {
                    match fut.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(val) => {
                            let g = val + 20;
                            self.state = MainState::SpawnLoop(1, vec![], g);
                        }
                    }
                }
                MainState::SpawnLoop(i, handles, _g) => {
                    if *i < 50000 {
                        let handle = tokio::spawn(PrintProTip::new(*i));
                        handles.push(handle);
                        *i += 1;
                    } else {
                        self.state = MainState::AwaitAll(0, std::mem::take(handles));
                    }
                }
                MainState::AwaitAll(idx, handles) => {
                    while *idx < handles.len() {
                        let handle = &mut handles[*idx];
                        match Pin::new(handle).poll(cx) {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(_) => {
                                *idx += 1;
                            }
                        }
                    }
                    self.state = MainState::Done;
                }
                MainState::Done => return Poll::Ready(()),
            }
        }
    }
}

//
// Entry Point
//

#[tokio::main]
async fn main() {
    let _ = MainFuture::new().await;
}
