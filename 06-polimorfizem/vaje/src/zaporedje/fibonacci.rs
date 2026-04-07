use std::ops::Add;

use super::Zaporedje;

// ============================================================
// FIBONACCIJEVO ZAPOREDJE
// ============================================================
// F_1 = 0, F_2 = 1, F_n = F_{n-1} + F_{n-2}

// pub: main.rs ustvarja FibonacciZaporedje
pub struct FibonacciZaporedje<T> {
    ime: String,
    f0: T,
    f1: T,
}

impl<T> FibonacciZaporedje<T> {
    // pub: main.rs kliče new
    pub fn new(ime: &str, f0: T, f1: T) -> Self {
        FibonacciZaporedje {
            ime: ime.to_string(),
            f0,
            f1,
        }
    }
}

impl<T> Zaporedje<T> for FibonacciZaporedje<T>
where
    T: Clone + Add<Output = T> + PartialEq,
{
    fn name(&self) -> &str {
        &self.ime
    }

    fn start(&self) -> T {
        self.f0.clone()
    }

    fn k_th(&self, k: u64) -> T {
        if k == 1 {
            return self.f0.clone();
        }
        if k == 2 {
            return self.f1.clone();
        }
        let mut prev = self.f0.clone();
        let mut curr = self.f1.clone();
        for _ in 2..k {
            let next = prev.clone() + curr.clone();
            prev = curr;
            curr = next;
        }
        curr
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
