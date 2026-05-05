use std::ops::{Deref, DerefMut};
use std::rc::Rc;

struct MyBox<T> {
    value: T,
    history: Vec<T>,
}

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox {
            value,
            history: vec![],
        }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: Clone> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.history.push(self.value.clone());
        &mut self.value
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!(
            "Preden izginem, naj povem, da sem se spremenil {}-krat",
            self.history.len()
        );
    }
}

#[derive(Clone)]
enum BoxList<T> {
    Nil,
    Cons (T, Box<BoxList<T>>)
}

enum RcList<T> {
    Nil,
    Cons (T, Rc<RcList<T>>)
}


fn main() {
    let x = Box::new(5);
    println!("Vrednost v škatli je {}", *x);
    let mut y = MyBox::new(5);
    println!("Vrednost v moji škatli je {}", *y);
    *y = 10;
    println!("Vrednost v moji škatli je {}", *y);
    *y = 20;
    println!("Vrednost v moji škatli je {}", *y);

    let mut xs : Box<BoxList<i32>> = Box::new(BoxList::Nil);
    let mut xss : BoxList<BoxList<i32>> = BoxList::Nil;
    let start = std::time::Instant::now();
    for i in 1..400 {
        let ys = xs.clone();
        xs = Box::new(BoxList::Cons(i, xs));
        xss = BoxList::Cons(*ys, Box::new(xss));
    }
    println!("Čas: {:?}", start.elapsed());


    let mut xs : Rc<RcList<i32>> = Rc::new(RcList::Nil);
    let mut xss : RcList<Rc<RcList<i32>>> = RcList::Nil;
    let start = std::time::Instant::now();
    for i in 1..8000 {
        let ys = Rc::clone(&xs);
        xs = Rc::new(RcList::Cons(i, xs));
        xss = RcList::Cons(ys, Rc::new(xss));
    }
    println!("Čas: {:?}", start.elapsed());
}
