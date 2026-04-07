mod aritmeticno;
mod combined;
mod fibonacci;
mod geometrijsko;
mod konstantno;
mod zamaknjeno;
mod zamaknjeno_dyn;

pub use aritmeticno::AritmeticnoZaporedje;
pub use combined::Combined;
pub use fibonacci::FibonacciZaporedje;
pub use geometrijsko::GeometrijskoZaporedje;
pub use konstantno::KonstantnoZaporedje;
pub use zamaknjeno::ZamaknjenoZaporedje;
pub use zamaknjeno_dyn::ZamaknjenoDyn;

// ============================================================
// ZNAČILNOST Zaporedje<T>
// ============================================================
// Zaporedje je splošen koncept: ima ime, začetni člen, k-ti člen in preverjanje vsebovanosti.

// pub: main.rs uporablja značilnost za izpis in delo z zaporedji.
// Metode v trait-u so avtomatično javne (kot variante enuma).
pub trait Zaporedje<T> {
    fn name(&self) -> &str;
    fn start(&self) -> T;
    fn k_th(&self, k: u64) -> T;
    fn contains(&self, value: &T) -> bool;
}

// Blanket impl: če Z implementira Zaporedje<T>, potem tudi &Z.
// To potrebujemo, ker ZamaknjenoZaporedje<Z> zahteva Z: Zaporedje<T>,
// in ko podamo referenco (&fib), je Z = &FibonacciZaporedje<i64>.
impl<T, Z: Zaporedje<T>> Zaporedje<T> for &Z {
    fn name(&self) -> &str {
        (*self).name()
    }
    fn start(&self) -> T {
        (*self).start()
    }
    fn k_th(&self, k: u64) -> T {
        (*self).k_th(k)
    }
    fn contains(&self, value: &T) -> bool {
        (*self).contains(value)
    }
}
