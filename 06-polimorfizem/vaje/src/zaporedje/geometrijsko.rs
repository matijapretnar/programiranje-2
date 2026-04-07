use std::ops::Mul;

use super::Zaporedje;

// ============================================================
// GEOMETRIJSKO ZAPOREDJE (generično)
// ============================================================
// a_n = a1 * q^(n-1)

// pub: main.rs ustvarja GeometrijskoZaporedje
pub struct GeometrijskoZaporedje<T> {
    ime: String,
    a1: T,
    q: T,
}

impl<T> GeometrijskoZaporedje<T> {
    // pub: main.rs kliče new
    pub fn new(ime: &str, a1: T, q: T) -> Self {
        GeometrijskoZaporedje {
            ime: ime.to_string(),
            a1,
            q,
        }
    }
}

impl<T> Zaporedje<T> for GeometrijskoZaporedje<T>
where
    T: Clone + Mul<Output = T> + PartialEq,
{
    fn name(&self) -> &str {
        &self.ime
    }

    fn start(&self) -> T {
        self.a1.clone()
    }

    fn k_th(&self, k: u64) -> T {
        // a_k = a1 * q^(k-1)
        let mut result = self.a1.clone();
        for _ in 0..(k - 1) {
            result = result * self.q.clone();
        }
        result
    }

    fn contains(&self, value: &T) -> bool {
        let mut k = 1;
        loop {
            if self.k_th(k) == *value {
                return true;
            }
            k += 1;
        }
    }
}
