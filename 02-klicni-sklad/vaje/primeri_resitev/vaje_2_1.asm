; Vaja 2.1: Iskanje najmanjšega števila
; Poišče minimum v seznamu in ga shrani v celico 'min'

JMP main

; Podatki
len:  DB 6              ; dolžina seznama
data: DB 45             ; elementi
      DB 12
      DB 78
      DB 5
      DB 34
      DB 89
min:  DB 0              ; sem pride rezultat

main:
    MOV A, data         ; A = naslov prvega elementa
    MOV B, [A]          ; B = trenutni minimum (prvi element)
    MOV C, [len]        ; C = števec elementov
    DEC C               ; že obdelali prvi element

find_min_loop:
    CMP C, 0            ; če ni več elementov
    JZ find_min_done
    INC A               ; naslednji element
    MOV D, [A]          ; D = trenutni element
    CMP D, B            ; primerjaj z minimumom
    JNB skip_update     ; če D >= B, preskoči
    MOV B, D            ; nov minimum
skip_update:
    DEC C               ; zmanjšaj števec
    JMP find_min_loop

find_min_done:
    MOV [min], B        ; shrani minimum
    HLT
