use std::ops::{Add, Mul, Sub};

use super::Zaporedje;
use crate::izraz::Izraz;

// ============================================================
// KOMBINIRANO ZAPOREDJE
// ============================================================
// Combined sprejme izraz s spremenljivkami in seznam zaporedij.
// i-ti člen kombiniranega zaporedja izračunamo tako, da v izraz
// vstavimo i-te člene vhodnih zaporedij (po imenu).

// pub: main.rs ustvarja Combined
pub struct Combined<'a, T> {
    ime: String,
    izraz: &'a Izraz<T>,
    zaporedja: Vec<&'a dyn Zaporedje<T>>,
}

impl<'a, T> Combined<'a, T> {
    // pub: main.rs kliče new
    pub fn new(ime: &str, izraz: &'a Izraz<T>, zaporedja: Vec<&'a dyn Zaporedje<T>>) -> Self {
        Combined {
            ime: ime.to_string(),
            izraz,
            zaporedja,
        }
    }
}

impl<T> Zaporedje<T> for Combined<'_, T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + PartialEq,
{
    fn name(&self) -> &str {
        &self.ime
    }

    fn start(&self) -> T {
        self.k_th(1)
    }

    fn k_th(&self, k: u64) -> T {
        // Evalviramo izraz: za vsako spremenljivko poiščemo zaporedje z enakim imenom
        // in vrnemo njegov k-ti člen.
        self.izraz.eval(&|ime| {
            for zap in &self.zaporedja {
                if zap.name() == ime {
                    return zap.k_th(k);
                }
            }
            panic!("Zaporedje z imenom '{}' ni bilo najdeno", ime);
        })
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
