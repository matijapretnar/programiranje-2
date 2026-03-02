; Vaja 2.7: Sito Eratostenovo
; Poišče praštevila do M in jih potisne na sklad
; Zaradi omejenega pomnilnika (256B) deluje za majhen M

JMP main

; M = zgornja meja (vključno)
M: DB 30

; Sito: 0 = praštevilo, 1 = sestavljeno
; Indeks = število, vrednost = oznaka
; sieve[0] in sieve[1] sta vedno 1 (nista praštevili)
sieve: DB 1
       DB 1
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0
       DB 0

; Pomožne spremenljivke
p: DB 0                 ; trenutno praštevilo
j: DB 0                 ; večkratnik
ptr: DB 0               ; kazalec v sieve

main:
    ; Inicializiraj sito - že narejeno z DB
    
    ; Zaženi sito
    MOV A, 2
    MOV [p], A          ; p = 2 (prvo praštevilo)
    
sieve_outer:
    ; Preveri, če p*p > M
    MOV A, [p]
    MOV B, A
    ; Izračunaj p*p z množenjem
    MOV C, 0            ; C = p*p
    MOV D, A            ; D = števec
mul_pp:
    CMP D, 0
    JZ mul_pp_done
    ADD C, A
    DEC D
    JMP mul_pp
mul_pp_done:
    ; C = p*p
    CMP C, [M]
    JA collect_primes   ; p*p > M -> končaj označevanje
    
    ; Preveri, če je p praštevilo (sieve[p] == 0)
    MOV A, sieve
    ADD A, [p]          ; A = naslov sieve[p]
    MOV B, [A]
    CMP B, 0
    JNZ sieve_next_p    ; ni praštevilo, preskoči
    
    ; Označi večkratnike p*p, p*p+p, p*p+2p, ...
    MOV A, [p]
    ; j = p*p (C že vsebuje to vrednost)
    MOV [j], C
    
mark_multiples:
    MOV A, [j]
    CMP A, [M]
    JA sieve_next_p     ; j > M -> naslednji p
    
    ; Označi sieve[j] = 1
    MOV B, sieve
    ADD B, A            ; B = naslov sieve[j]
    MOV A, 1
    MOV [B], A
    
    ; j = j + p
    MOV A, [j]
    ADD A, [p]
    MOV [j], A
    JMP mark_multiples
    
sieve_next_p:
    MOV A, [p]
    INC A
    MOV [p], A
    JMP sieve_outer

collect_primes:
    ; Zberi praštevila in jih potisni na sklad
    MOV A, 2            ; začni pri 2
    
collect_loop:
    CMP A, [M]
    JA done             ; če A > M, končaj
    
    ; Preveri, če je A praštevilo
    MOV B, sieve
    ADD B, A            ; B = naslov sieve[A]
    MOV C, [B]
    CMP C, 0
    JNZ collect_next    ; ni praštevilo
    
    ; Je praštevilo - potisni na sklad
    PUSH A
    
collect_next:
    INC A
    JMP collect_loop
    
done:
    HLT
    
; Po izvajanju so praštevila na skladu:
; 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
