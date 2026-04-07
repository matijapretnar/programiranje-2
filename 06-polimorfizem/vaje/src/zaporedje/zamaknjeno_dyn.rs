use super::Zaporedje;

// ============================================================
// ZAMAKNJENO ZAPOREDJE (alternativa z dyn - dynamic dispatch)
// ============================================================
// Alternativna verzija, ki namesto generičnega Z uporabi &dyn Zaporedje<T>.
//
// RAZLIKA:
// - ZamaknjenoZaporedje<Z>: Z je generičen tip, ki ga prevajalnik
//   razreši ob prevajanju (statični dispatch). Za vsak konkreten Z se
//   generira svoja kopija kode. Ni potrebe po življenjskih dobah v definiciji
//   strukture — če podamo referenco, Rust sam sklepa življenjsko dobo.
//
// - ZamaknjenoDyn<'a, T>: hrani &dyn Zaporedje<T>, trait object.
//   Prevajalnik ne ve, kateri konkreten tip je za referenco — klic metod
//   gre prek vtable (dinamični dispatch). Potrebujemo eksplicitno življenjsko
//   dobo 'a, ker struct hrani referenco.
//
// KDAJ UPORABITI KATERO:
// - Generična verzija (Z) je hitrejša (brez vtable) in preprostejša za uporabo.
// - dyn verzija je potrebna, ko želimo hraniti zaporedja RAZLIČNIH tipov
//   v isti strukturi (npr. Vec<&dyn Zaporedje<T>>), ker generik Z dovoli
//   le en konkreten tip naenkrat.

pub struct ZamaknjenoDyn<'a, T> {
    zaporedje: &'a dyn Zaporedje<T>,
    zamik: u64,
}

impl<'a, T> ZamaknjenoDyn<'a, T> {
    pub fn new(zaporedje: &'a dyn Zaporedje<T>, n: u64) -> ZamaknjenoDyn<'a, T> {
        ZamaknjenoDyn {
            zaporedje,
            zamik: n,
        }
    }
}

impl<T: PartialEq> Zaporedje<T> for ZamaknjenoDyn<'_, T> {
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
