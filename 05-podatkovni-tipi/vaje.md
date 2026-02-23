# Strukture

S pomočjo ukaza `cargo` začnite nov projekt po imenu `structs`.

## Osnovne strukture

Na vajah bomo uporabo struktur v rustu spoznali prek zaporedij in njihove uporabe.

1. Definirajte strukturo `ArtimeticnoZaporedje`, ki naj predstavlja aritmetično zaporedje.
Zaradi enostavnosti se omejite na celoštevilska zaporedja, ki pa so lahko negativna.
2. Razmislite, kakšne stvari vse lahko zaporedje počne (kaj lahko od njega zahtevamo), pri tem ne glejte naprej.
Različne zahteve zapišite v slovenščini (angleščini) kot komentarje in obrazložite, zakaj jih potrebujemo.
3. Implementirajte naslednje metode (po potrebi se vrnite na točko 1 in spremenite definicijo):
   - `new` za ustvarjanje novega zaporedja.
   - `next` za pridobitev trenutnega člena zaporedja in zaporedje premakne naprej.
   Ukaz `(a.next();a.next())` naj tako za zaporedje z začetkom 1 in korakom 1 vrne `(1,2)`.
   - `n_th` za pridobitev n-tega člena zaporedja.
   - `reset` za ponastavitev zaporedja na začetek, tako da bo naslednji  `next` vrnil prvi člen zaporedja.
   - `current` za pridobitev trenutnega člena zaporedja, vendar brez premika na naslednjega.
   - `sum`, ki sešteje prvih `n` členov zaporedja, a zaporedje pusti v nespremenjenem stanju.
   Pri tem ne uporabite formule za vsoto aritmetičnega zaporedja.
4. Od tu naprej ne smete več spreminjati definicije strukture.
5. Definirajte metodo `vsota`, ki sprejme dve zaporedji in vrne novo zaporedje, ki je vsota obeh vhodnih zaporedij.
Ali je to mogoče narediti za splošna zaporedja?
Pri implementaciji bodite čim bolj učinkoviti (ne kopirajte stvari, ki jih ne rabite).
6. Definirajte metodo `produkt`, ki sprejme dve zaporedji in vrne novo zaporedje, ki je produkt obeh vhodnih zaporedij, argumentirajte, ali se to sploh da narediti na lep način.
7. Ponovite vse točke od prej za geometrijska zaporedja.

## AST

Matematične izraze (in tudi programe) enostavno predstavimo z drevesi, ki jih tipično imenujemo abstraktno sintaktično drevo (AST).

Najprej definirajte sledeči strukturi:
```rust
enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta(u32),
    Operacija(Box<Izraz>, BinOperacija, Box<Izraz>),
}
```
1. Poskusite napisati tip za `Izraz` brez uporabe `Box` in skupaj s prevajalnikom razmislite, zakaj to ne deluje.
1. Razmislite, ali je potrebno v zapis izraza dodati tudi oklepaje, ali je dovolj že to, da drevo pravilno predstavlja izraz.
1. Zapišite primer izrazov za:
    - `1 + (2 * 3)`
    - `(1 + 2) * 3`
    - `1 + 2 + 3`
    - `5**2 + 3**2`
    - `5 * 5 + 4**2`
1. Implementirajte metodo `eval`, ki izračuna vrednost izraza.
1. Implementirajte metodo `collect`, ki vrne število konstant v izrazu.
1. Implementirajte za izpisovanje `izpis`, ki vrne izraz v obliki `(a + b) * c`.
Poskrbite, da boste pravilno izpisali oklepaje, vendar se ne obremenjujte, če izpište kakšen dodaten oklepaj.
1. Napišite nekaj primernih testov za metode `eval`, `collect` in `izpis`.


<!-- ## Razširitve AST

1. AST dopolnite z unarno opracijo `UnMinus`, ki predstavlja `-x` za nek izraz `x`.
1. Dodajte še binarne operacije
    - `Pow`, ki predstavlja potenciranje.
    - `Mod`, ki predstavlja ostanek pri deljenju.
3. Ternarno operacijo `PowMod`, ki predstavlja `a**b % c`.

## Razširitve zaporedij -->