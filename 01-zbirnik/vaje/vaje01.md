# Vaje 01

Simulator: https://schweigi.github.io/assembler-simulator/

Vsebina: Osnovna orodja - registri `A,B,C,D`, skoki (`JMP`, `JZ`, `JNB`, ...), primerjave (`CMP`) in zanke.

Namen: spoznati osnovne operacije z registri, pogoje in zanke ter iterativne algoritme (brez rekurzije in brez zahtev po klicih funkcij).

Naloge

1. Ostanek

Napišite program, ki v register `A` zapiše ostanek pri deljenju vrednosti, ki je trenutno v `A`, z vrednostjo v `B`.

<details>
    <summary>Namig</summary>

    uporabite zanko in odštevanje (`SUB`) dokler `A < B`.

</details>
 

2. Deljenje (kvocient in ostanek)

Napišite program, ki izračuna kvocient in ostanek pri `A / B`. Kvocient naj bo v `C`, ostanek v `A` po koncu.

3. Zaporedje v pomnilnik

Zapišite zaporedje celih števil od 13 do 42 v zaporedne pomnilniške celice (brez sklada).

  <details>
        <summary>Namig 1</summary>

        v `B` shrani mesto, kjer se bo zaporedje začelo (poskusi recimo s 3), nato pa si pomagaj z `MOV [B], ...`

</details>

 <details>
        <summary>Alternativni namig</summary>

        v pomnilniku si rezerviraj prostor za 29 števil.

</details>

  <details>
        <summary>Izziv</summary>

        V namigu 1 pride do težave, da program prepiše samega sebe in ne naredi tega kar želimo. Kako bi zagotovili, da se program ne bo nikoli prepisal, tudi če vmes dodamo še nakaj drugih ukazov?

</details>
 

4. Menjava vsebin pomnilniških celic

V register `A` in `B` sta shranjena naslova dveh pomnilniških mest. Napišite program, ki zamenja vsebini na naslovih `[A]` in `[B]` z uporabo začasnih registrov. Vhodne podatke preberi kot:

```
input_A: DB 11
input_B: DB 22
```

5. Preprosta vsota

Napišite kratek fragment, ki izračuna `A + B` in rezultat pusti v `A`.

6. Iterativni Fibonacci

Napišite zaporedje ukazov, ki izračuna n-to Fibonaccijevo število (iterativno, brez rekurzije). Vhod: `n` v registru `B`; izhod: `A = F(n)`.

  <details>
        <summary>Namig</summary>

        To ste delali na predavanjih.

</details>
  <details>
       <summary>Izziv</summary>

        Izračunajte 14. člen fibbonacijevega zaporedja

</details>
 

7. Menjava registrov brez začasnega registra

Napišite program, ki zamenja vsebini registrov `A` in `B` brez uporabe tretjega registra ali pomnilniške celice. Razmislite o primerih, ko sta enaki.

  <details>
       <summary>Namig</summary>

        XOR swap

</details>
 

8. Največji izmed dveh

Napišite program, ki primerja `A` in `B` in v `A` shrani največjo od njiju.

  <details>
       <summary>Namig</summary>

        Uporabite `CMP` in pogojne skoke.

</details>
