use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct MemoryBox {
    contents: i32,
    history: Vec<i32>,
}

impl MemoryBox {
    fn new(i: i32) -> Self {
        MemoryBox {
            contents: i,
            history: vec![],
        }
    }
}

impl Deref for MemoryBox {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.contents
    }
}

impl DerefMut for MemoryBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.history.push(self.contents);
        &mut self.contents
    }
}

impl Drop for MemoryBox {
    fn drop(&mut self) {
        println!("All important moments flashing before my eyes...");
        for i in self.history.iter() {
            println!("{i}, aaahh....")
        }
    }
}

#[derive(Clone, Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let mut n = MemoryBox::new(42);
    for i in 1..10 {
        *n = *n + i;
        println!("{}", *n);
    }

    let mut xss = Vec::new();
    let mut xs = Rc::new(Nil);
    // pri daljših seznamih nastopi težava s prekoračitvijo sklada
    // zaradi rekurzivnega drop-a
    for x in 1..20000 {
        xss.push(Rc::clone(&xs));
        xs = Rc::new(Cons(x, xs));
    }
}
