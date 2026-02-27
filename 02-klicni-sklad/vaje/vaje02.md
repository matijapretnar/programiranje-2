# Vaje 02

Simulator: https://schweigi.github.io/assembler-simulator/

Vsebina: sklad, funkcije.

Naloge

1. Iskanje najmanjšega števila

V pomnilniku naredite prostor za spremenljivko `min`. Shranite v zaporedne celice naključnih nekaj števil (natipkajte na roke, ni treba implementirati funkcije za naključna števila) in dolžino zaporedja.

Napišite program, ki v celici `min` zapiše najmanjšo vrednost v seznamu.

2. Funkcija `poisci_minimum`

Napišite funkcijo `poisci_minimum`, ki kot parametra uporablja registre: `A` naj kaže na začetek rezine, `C` na konec (naslednji naslov za zadnji element). Funkcija naj v `B` shrani indeks najmanjšega elementa (relativno 0..n-1) in ohrani vse registre razen `B`.

3. Menjava pomnilniških celic s kazalniki

Napišite podprogram `swap_mem`, ki sprejme dva naslova v registrih `A` in `B` in zamenja vsebini pomnilniških mest `[A]` in `[B]`.

4. Sortiranje — funkcija `uredi`

Napišite funkcijo `uredi`, ki uredi elemente v rezini `[A:C)` od najmanjšega do največjega (izvajajte urejanje z izbiranjem). Funkcija naj ohrani vse registre in omogoči ponovno uporabo.

5. Praštevila — generator

Napišite program, ki izračuna prvih N praštevil (N nastavljeno v pomnilniku ali registru) in jih zaporedno potiska na sklad. Izziv: poiščite način, da zmanjšate število deljenj ali pospešite algoritem.

6. Obrni seznam v pomnilniku

Napišite program, ki v pomnilniku obrne seznam dolžine N (podan z `DB`) z uporabo dveh kazalcev — začetek in konec.

7. Sito Eratostenovo (izziv)

Implementirajte sito do M in potisnite praštevila na sklad ali jih zapišite v pomnilnik.
