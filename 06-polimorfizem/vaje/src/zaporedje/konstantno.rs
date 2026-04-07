use super::Zaporedje;

// ============================================================
// KONSTANTNO ZAPOREDJE
// ============================================================
// a_n = c za vsak n

// pub struct: main.rs ustvarja KonstantnoZaporedje.
// polja so private
pub struct KonstantnoZaporedje<T> {
    ime: String,
    vrednost: T,
}

impl<T> KonstantnoZaporedje<T> {
    // pub fn: main.rs kliče new za ustvarjanje zaporedij
    pub fn new(ime: &str, vrednost: T) -> Self {
        KonstantnoZaporedje {
            ime: ime.to_string(),
            vrednost,
        }
    }
}

// Metode v impl Trait NE potrebujejo pub — vidnost podedujejo od trait-a.
impl<T: Clone + PartialEq> Zaporedje<T> for KonstantnoZaporedje<T> {
    fn name(&self) -> &str {
        &self.ime
    }

    fn start(&self) -> T {
        self.vrednost.clone()
    }

    fn k_th(&self, _k: u64) -> T {
        self.vrednost.clone()
    }

    fn contains(&self, value: &T) -> bool {
        *value == self.vrednost
    }
}
