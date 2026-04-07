// use std::marker::PhantomData;

use super::Zaporedje;

// ============================================================
// ZAMAKNJENO ZAPOREDJE
// ============================================================
// Vzame obstoječe zaporedje in ga zamakne za n mest.
// k-ti člen zamaknjenega = (k+n)-ti člen originalnega.
// Z je generičen tip — lahko je &AritmeticnoZaporedje<i64>, &FibonacciZaporedje<i64>, itd.

// pub: main.rs uporablja zamaknjeno_zaporedje (funkcija vrne ta struct)
pub struct ZamaknjenoZaporedje<Z> {
    zaporedje: Z,
    zamik: u64,
    // _marker: PhantomData<T>,
}

impl<Z> ZamaknjenoZaporedje<Z> {
    // pub: main.rs kliče to metodo
    pub fn new(zaporedje: Z, n: u64) -> ZamaknjenoZaporedje<Z> {
        ZamaknjenoZaporedje {
            zaporedje,
            zamik: n,
        }
    }
}

impl<T: PartialEq, Z: Zaporedje<T>> Zaporedje<T> for ZamaknjenoZaporedje<Z> {
    fn name(&self) -> &str {
        self.zaporedje.name()
    }

    fn start(&self) -> T {
        self.zaporedje.k_th(self.zamik + 1)
    }

    fn k_th(&self, k: u64) -> T {
        self.zaporedje.k_th(k + self.zamik)
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
