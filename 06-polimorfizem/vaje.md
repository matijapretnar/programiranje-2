# Generiki in značilnosti

Za naloge, ki sledijo, boste potrebovali strukturo aritmetičnega zaporedja,
ki ste ga definirali zadnjič, in strukturo izrazov ter binarnih operacij.

Projekt organizirajte v ločene module:

- `izraz.rs` — generični izrazi (AST)
- `zaporedje.rs` — značilnost `Zaporedje` in implementacije zaporedij
- `main.rs` — demonstracija in testi

## 1. Generični izrazi

1. Popravite `Izraz` tako, da bo konstanta v izrazu poljubnega tipa `T`.
   Dodajte tudi varianto `Spremenljivka(String)`.
2. Katerim značilnostim mora zadoščati tip `T`, da bo imela metoda `eval` smisel?
   Kaj pa `collect` in `izpis`?
3. Za `Izraz<T>` implementirajte značilnost `Display`.

## 2. Značilnost `Zaporedje<T>`

4. Definirajte značilnost `Zaporedje<T>`, ki predstavlja poljubno zaporedje in ima metode:
   - `name(&self) -> &str` — ime zaporedja
   - `start(&self) -> T` — prvi člen
   - `k_th(&self, k: u64) -> T` — k-ti člen (1-indeksirano)
   - `contains(&self, value: &T) -> bool` — ali zaporedje vsebuje dano vrednost
5. Definirajte naslednja zaporedja in za vsakega metodo `new`, ki ustvari to zaporedje. Za vsakega implementirajte `Zaporedje<T>`:
   - **Konstantno zaporedje** — vsi členi so enaki.
   - **Aritmetično zaporedje** — `a_k = a1 + (k-1) * d`. Popravite definicijo od zadnjič, da bo generična za poljuben tip `T`. Ugotovite, katerim značilnostim mora `T` zadoščati.
   - **Geometrijsko zaporedje** — `a_k = a1 * q^(k-1)`.
   - **Fibonaccijevo zaporedje** — `F_1 = f0, F_2 = f1, F_k = F_{k-1} + F_{k-2}`.
6. Za `AritmeticnoZaporedje<T>` implementirajte značilnost `PartialEq`.
   Kdaj sta dve aritmetični zaporedji enaki?
7. Definirajte metodo `zmnozi`, ki zmnožek dveh aritmetičnih zaporedij izračuna tako,
   da zmnoži začetna člena in diferenci.
8. Definirajte `ZamaknjenoZaporedje<Z>`, ki sprejme zaporedje generičnega tipa `Z`
   in število `n` ter vrne novo zaporedje, ki se začne z `n`-tim členom vhodnega zaporedja.
   Implementirajte `Zaporedje<T>` za `ZamaknjenoZaporedje<Z>` kjer `Z: Zaporedje<T>`.

## 3. Kombinirano zaporedje (`dyn` in življenjske dobe)

9. Definirajte zaporedje `Combined`, ki sprejme aritmetični izraz (s spremenljivkami)
   in seznam zaporedij ter vrne kombinirano zaporedje,
   kjer je `i`-ti člen izračunan z uporabo izraza in vrednosti členov iz vhodnih zaporedij.
   Ker seznam vsebuje zaporedja **različnih tipov**, uporabite `Vec<&dyn Zaporedje<T>>`.
   Razmislite, zakaj tu potrebujete `dyn` in življenjske dobe (`'a`).

## 4. Testiranje

10. Ustvarite nekaj aritmetičnih zaporedij različnih tipov (`i64`, `f64`) in testirajte operacije na njih.
11. Preizkusite `zamaknjeno_zaporedje` na Fibonaccijevem zaporedju.
12. Preizkusite generične izraze z različnimi tipi in spremenljivkami.
13. Preizkusite `Combined` zaporedje — npr. izraz `a + 2 * g` z aritmetičnim in geometrijskim zaporedjem.
