fn primer1() {
    let v1 = vec![2, 3, 5, 7, 11, 13];
    let f = |x| x + 1;
    let v2 : Vec<i32> = v1.iter().map(f).collect();
    println!("{:?}", v2)
}

// struct Podatki_za_f_iz_primera2 {
//     a: &Box<i32>
// }
// impl Closure for Podatki_za_f_iz_primera2 {
//     fn call(self, x: &i32) {
//         x + *self.a
//     }
// }

fn primer2() {
    let v1 = vec![2, 3, 5, 7, 11, 13];
    let a = Box::new(10);
    // let okolje_f = struct { a = &a }
    let f = |x| x + *a;
    // f = Podatki_za_f_iz_primera2 {
    //    a = a
    // }
    let v2 : Vec<i32> = v1.iter().map(f).collect();
    println!("{:?}", v2)
}

// struct Podatki_za_f_iz_primera3 {
//     a: &mut Vec<i32>
// }

fn primer3() {
    let v1 = vec![2, 3, 5, 7, 11, 13];
    let mut w: Vec<i32> = vec![];
    let mut f = |x| { w.push(10 * x) };
    // let okolje_f = struct { w = &mut a }
    for x in v1 {
        f(x);
    }
    println!("{:?}", w)
}

struct Podatki_za_f_iz_primera4 {
    a: Box<i32>
}

fn primer4() {
    let v1 = vec![2];
    let a = Box::new(10);
    let f = |x| { let b = a; x + *b };
    // ne dela, ker bi f poklicali več kot enkrat
    // let v2 : Vec<i32> = v1.iter().map(f).collect();
    let b = f(32);
    // let c = f(32);
    println!("{:?}", b)
}

fn main() {
    let x = 10;
    let v = [(|a| {a + x}), (|a| {a + x})];
}
