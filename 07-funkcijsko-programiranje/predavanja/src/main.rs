fn skalarni_produkt_zanka(vec1 : Vec<f64>, vec2 : Vec<f64>) -> f64 {
    let mut vsota = 0.;
    let mut i = 0;
    while i < vec1.len() {
        i += 1;
        vsota += vec1[i] * vec2[i]
    };
    vsota
}

fn zmnozi((x1, x2): (&f64, &f64)) -> f64 {
    x1 * x2
}

fn skalarni_produkt_iter(vec1 : Vec<f64>, vec2 : Vec<f64>) -> f64 {
    vec1.iter().zip(vec2.iter())
               .map(|(x1, x2)| x1 * x2)
               .sum()
}

fn primer1() {
    // struct TipSamoZaFunkcijoIzPrimera1 {
    // }
    // impl Fn(i32) -> i32 for TipSamoZaFunkcijoIzPrimera1 {
    //    fn call(&self, x: i32) -> i32 {
    //        x + 1
    //    }
    // }
    let f = |x| x + 1;
    println!("{}", f(1));
    println!("{}", f(2));
    println!("{}", f(3));
}


fn primer2() {
    let a = String::from("test");
    // struct TipSamoZaFunkcijoIzPrimera2 {
    //     a: &String
    // }
    // impl Fn(String) for TipSamoZaFunkcijoIzPrimera2 {
    //     fn call(&self, s: String) {
    //         println!("{}{}", s, self.a)
    //     }
    // }
    let f = |s| println!("{}{}", s, a);
    f(String::from("a"));
    f(String::from("b"));
    f(String::from("c"));
    println!("{a}")
}

fn primer3() {
    let stevila  = vec![10, 20, 30];
    let mut funkcije = vec![];
    for x in stevila.iter() {
        // struct TipSamoZaFunkcijoIzPrimera3 {
        //     x: i32
        // }
        funkcije.push(|y| x.clone() * y);
        // lahko tudi:
        // funkcije.push(move |y| x * y);
    }
    for f in funkcije.iter() {
        println!("{}", f(7))
    }
}

fn primer4() {
    let mut a = String::from("test");
    // struct TipSamoZaFunkcijoIzPrimera4 {
    //     a: &mut String
    // }
    // impl FnMut(String) for TipSamoZaFunkcijoIzPrimera4 {
    //     fn call(&mut self, s: String) {
    //         self.a.push_str("t");
    //         println!("{}{}", s, self.a)
    //     }
    // }
    let mut f = |s| {
        a.push_str("t");
        println!("{}{}", s, a)
    };
    f(String::from("a"));
    f(String::from("b"));
    f(String::from("c"));
    println!("{a}")
}

fn primer5() {
    let a = String::from("test");
    // struct TipSamoZaFunkcijoIzPrimera5 {
    //     a: String
    // }
    let f = |s| {
        let b = a;
        println!("{}{}", s, b)
    };
    f(String::from("a"));
    // spodnje ne delajo, ker je f prevzel lastništvo nad a, zato ga ne more več uporabiti
    // f(String::from("b"));
    // f(String::from("c"));
    // println!("{a}")
}

fn main() {
    let mut s = String::from("Hello");
    let mut add_suffix = || s.push_str(" world");
    println!("{s}");
    add_suffix();
}