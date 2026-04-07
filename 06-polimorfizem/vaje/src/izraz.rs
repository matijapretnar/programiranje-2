use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

// ============================================================
// GENERIČNI AST (Abstraktno sintaktično drevo)
// ============================================================
// Izraz je generičen: konstanta je tipa T (ne nujno u32).

// pub: main.rs mora uporabljati BinOperacija pri sestavljanju izrazov.
// Variante enuma so avtomatično javne, ko je enum pub.
#[derive(Debug)]
pub enum BinOperacija {
    Plus,
    Minus,
    Times,
}

// pub: main.rs mora ustvarjati in uporabljati izraze.
#[derive(Debug)]
pub enum Izraz<T> {
    Konstanta(T),
    Spremenljivka(String),
    Operacija(Box<Izraz<T>>, BinOperacija, Box<Izraz<T>>),
}

impl<T> Izraz<T> {
    // pub: pomožne funkcije za sestavljanje izrazov iz main.rs
    pub fn konst(v: T) -> Self {
        Izraz::Konstanta(v)
    }

    pub fn spr(ime: &str) -> Self {
        Izraz::Spremenljivka(ime.to_string())
    }

    pub fn op(left: Izraz<T>, op: BinOperacija, right: Izraz<T>) -> Self {
        Izraz::Operacija(Box::new(left), op, Box::new(right))
    }
}

// eval potrebuje: Add, Sub, Mul (za operacije) + Clone (za kopiranje vrednosti)
// Spremenljivke evalviramo z lookup funkcijo.
impl<T> Izraz<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Clone,
{
    // pub: main.rs kliče eval, eval_brez_spr
    // An impl Trait in a type-position can be used to designate a type
    // that implements a trait called Trait.
    pub fn eval(&self, lookup: &impl Fn(&str) -> T) -> T {
        match self {
            Izraz::Konstanta(v) => v.clone(),
            Izraz::Spremenljivka(ime) => lookup(ime),
            Izraz::Operacija(l, op, r) => {
                let lv = l.eval(lookup);
                let rv = r.eval(lookup);
                match op {
                    BinOperacija::Plus => lv + rv,
                    BinOperacija::Minus => lv - rv,
                    BinOperacija::Times => lv * rv,
                }
            }
        }
    }

    // Eval brez spremenljivk (panic če naleti na spremenljivko).
    pub fn eval_brez_spr(&self) -> T {
        self.eval(&|ime| panic!("Neznana spremenljivka: {}", ime))
    }
}

// collect ne potrebuje nobenih posebnih značilnosti za T.
impl<T> Izraz<T> {
    // pub: main.rs kliče collect
    pub fn collect(&self) -> u32 {
        match self {
            Izraz::Konstanta(_) => 1,
            Izraz::Spremenljivka(_) => 0,
            Izraz::Operacija(l, _, r) => l.collect() + r.collect(),
        }
    }
}

// izpis / ToString potrebuje: Display (za izpis konstant)
impl<T: Display> Izraz<T> {
    // pub: main.rs kliče izpis
    pub fn izpis(&self) -> String {
        match self {
            Izraz::Konstanta(v) => v.to_string(),
            Izraz::Spremenljivka(ime) => ime.clone(),
            Izraz::Operacija(l, op, r) => {
                let op_str = match op {
                    BinOperacija::Plus => "+",
                    BinOperacija::Minus => "-",
                    BinOperacija::Times => "*",
                };
                format!("({} {} {})", l.izpis(), op_str, r.izpis())
            }
        }
    }
}

// Display za Izraz<T> — zahteva Display za T.
// Display avtomatično zagotovi tudi ToString.
impl<T: Display> std::fmt::Display for Izraz<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.izpis())
    }
}
