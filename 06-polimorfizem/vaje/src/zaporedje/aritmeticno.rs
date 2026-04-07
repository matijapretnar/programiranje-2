use std::ops::{Add, Mul};

use super::Zaporedje;

// ============================================================
// ARITMETIČNO ZAPOREDJE (generično)
// ============================================================
// a_n = a1 + (n-1) * d
// T mora podpirati seštevanje, (množenje, kloniranje) in primerjavo.

// pub: main.rs ustvarja in uporablja AritmeticnoZaporedje
pub struct AritmeticnoZaporedje<T> {
    ime: String,
    a1: T,
    d: T,
}

impl<T> AritmeticnoZaporedje<T> {
    // pub: main.rs kliče new
    pub fn new(ime: &str, a1: T, d: T) -> Self {
        AritmeticnoZaporedje {
            ime: ime.to_string(),
            a1,
            d,
        }
    }
}

// Za metode zaporedja potrebujemo:
// - Clone: da lahko kopiramo vrednosti (a1, d) - če delamo samo s
// tipi ki živijo na stacku, lahko Clone nadomestimo s Copy (poskusite sami)
// - Add<Output=T>: za a1 + d + d + ...
// - PartialEq: za contains (primerjava členov)
// Ne potrebujemo Mul ali From<u64> — namesto tega uporabimo zanko
// (ponavljajoče seštevanje), kar deluje za vsak tip s seštevanjem.

impl<T> Zaporedje<T> for AritmeticnoZaporedje<T>
where
    T: Clone + Add<Output = T> + PartialEq,
{
    fn name(&self) -> &str {
        &self.ime
    }

    fn start(&self) -> T {
        self.a1.clone()
    }

    fn k_th(&self, k: u64) -> T {
        // a_k = a1 + (k-1) * d, izračunano s ponavljajočim seštevanjem
        let mut result = self.a1.clone();
        for _ in 1..k {
            result = result + self.d.clone();
        }
        result
    }

    fn contains(&self, value: &T) -> bool {
        // Semidecidable: če element je v zaporedju, ga bomo našli.
        // Če ga ni, se zanka ne ustavi. - to se da narediti bolj
        // učinkovito za aritmetična zaporedja.
        // ta implementacija pa bi lahko bila definirana že kot default?
        let mut k = 1;
        loop {
            if self.k_th(k) == *value {
                return true;
            }
            k += 1;
        }
    }
}

// PartialEq: dve aritmetični zaporedji sta enaki, če imata enak a1 in d.
impl<T: PartialEq> PartialEq for AritmeticnoZaporedje<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a1 == other.a1 && self.d == other.d
    }
}

// Zmnožek dveh aritmetičnih zaporedij: zmnožimo a1 in d.
// (To je definicija iz vaje, da ostane rezultat aritmetično zaporedje.)
impl<T: Clone + Mul<Output = T>> AritmeticnoZaporedje<T> {
    // pub: main.rs kliče zmnozi
    pub fn zmnozi(&self, other: &Self) -> Self {
        AritmeticnoZaporedje {
            ime: format!("({} * {})", self.ime, other.ime),
            a1: self.a1.clone() * other.a1.clone(),
            d: self.d.clone() * other.d.clone(),
        }
    }
}
