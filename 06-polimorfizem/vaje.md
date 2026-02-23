# Generiki in značilnosti

Za naloge, ki sledijo, boste potrebovali strukturo aritmetičnega zaporedja,
ki ste ga definirali zadnjič in strukturo izrazov in binarnih operacij.

- Popravite definicijo aritmetičnega zaporedja od zadnjič, da bo delovala za poljubne tipe `T`. Ali imajo vse metode od zadnjič smisel?
- Ugotovi, katerim značilnostim mora zadoščati tip `T`, da bodo imele metode od zadnjič smisel. Za tak tip `T`, popravi implementacijo metod od zadnjič. Zmnožek dveh aritmetičnih zaporedij definiramo tako, da zmnožimo začetna člena in diferenci (zato da bo produkt dveh aritmetičnih zaporedij tudi aritmetično zaporedje).
- Implementirajte značilnost `PartialEq` za aritmetična zaporedja.

- Definirajte značilnost `Zaporedje<T>`, ki predstavlja poljubno zaporedje in ima metode `name`, `start`, `k_th` in `contains`.
- Definirajte: konstantno zaporedje, aritmetično zaporedje, geometrijsko zaporedje, zaporedje Fibonaccijevih števil.
- Definirajte `zamaknjeno_zaporedje`, ki sprejme zaporedje in število `n` in vrne zaporedje, ki se začne z `n`-tim členom vhodnega zaporedja.
- Definirajte zaporedje `Combined`, ki sprejme aritmetični izraz (s spremenljivkami) in seznam zaporedij (s pravilnimi imeni) in vrne kombinirano zaporedje, kjer je `i`-ti člen izračunan z uporabo izraza in vrednosti členov iz vhodnih zaporedij.
  
- Popravite `Izraz` tako, da bo konstanta v izrazu poljubnega tipa `T`.
- Katerim značilnostim mora zadoščati tip `T`, da bo imela metoda `eval` smisel? Kaj pa `collect ` in `izpis`?
- Za `Izraz` implementirajte značilnost `ToString`.

- Ustvarite nekaj aritmetičnih zaporedij, in testirajte operacije na njih.
