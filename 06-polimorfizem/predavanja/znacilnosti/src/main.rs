use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;

fn najvecji<T>(v: &Vec<T>) -> Option<&T>
where
    T: PartialOrd,
{
    let mut najvecji = None;
    for x in v {
        match najvecji {
            None => najvecji = Some(x),
            Some(m) => {
                if m < x {
                    najvecji = Some(x)
                }
            }
        }
    }
    najvecji
}

#[derive(Debug, Clone, Copy)]
struct Ulomek {
    stevec: i32,
    imenovalec: i32,
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Ulomek {
    fn new(stevec: i32, imenovalec: i32) -> Self {
        let gcd = gcd(stevec.abs(), imenovalec.abs());
        Ulomek {
            stevec: stevec / gcd,
            imenovalec: imenovalec / gcd,
        }
    }
}

impl PartialEq for Ulomek {
    fn eq(&self, other: &Self) -> bool {
        self.stevec * other.imenovalec == self.imenovalec * other.stevec
    }
}

impl Eq for Ulomek {}

trait DelnaEnakost {
    fn enako(&self, other: &Self) -> bool {
        !self.nienako(other)
    }
    fn nienako(&self, other: &Self) -> bool {
        !self.enako(other)
    }
}

impl DelnaEnakost for i32 {}

impl PartialOrd for Ulomek {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ulomek {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.stevec * other.imenovalec).cmp(&(self.imenovalec * other.stevec))
    }
}

impl Display for Ulomek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.stevec, self.imenovalec)
    }
}

trait ImaVelikost {
    fn velikost() -> usize;
}

impl ImaVelikost for i32 {
    fn velikost() -> usize {
        4
    }
}
impl ImaVelikost for i64 {
    fn velikost() -> usize {
        8
    }
}
impl<T1: ImaVelikost, T2> ImaVelikost for (T1, T2)
where
    T2: ImaVelikost,
{
    fn velikost() -> usize {
        T1::velikost() + T2::velikost()
    }
}

trait Group {
    const UNIT: Self;
    fn inverse(&self) -> Self;
    fn multiply(&self, other: &Self) -> Self;
    fn divide(&self, other: &Self) -> Self
    where
        Self: Sized,
    {
        self.multiply(&other.inverse())
    }
    fn is_inverse(a: &Self, b: &Self) -> bool
    where
        Self: PartialEq + Sized,
    {
        a.multiply(b) == Self::UNIT
    }
}

impl Group for Ulomek {
    const UNIT: Ulomek = Ulomek {
        stevec: 1,
        imenovalec: 1,
    };

    fn inverse(&self) -> Self {
        Ulomek::new(self.imenovalec, self.stevec)
    }

    fn multiply(&self, other: &Self) -> Self {
        Ulomek::new(
            self.stevec * other.stevec,
            self.imenovalec * other.imenovalec,
        )
    }
}

fn main() {
    // println!("Ali sta 6 in 7 enaki?: {}", 6.enako(&7));
    let u1 = Ulomek::new(1, 2);
    let u2 = Ulomek::new(2, 4);
    if u1 == u2 {
        println!("{u1} == {u2}");
    } else {
        println!("{u1} != {u2}");
    }
    println!("Kvocient ulomkov {u1} in {u2} je {}", u1.divide(&u2));
    if Ulomek::is_inverse(&u1, &u2) {
        println!("Ulomka {u1} in {u2} sta si inverzna.")
    }
}
