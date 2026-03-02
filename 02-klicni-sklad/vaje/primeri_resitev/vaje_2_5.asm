; Vaja 2.5: Generator praštevil
; Izračuna prvih N praštevil in jih potisne na sklad
; N je shranjen v pomnilniku

JMP main

N: DB 8                 ; koliko praštevil generirati
count: DB 0             ; števec najdenih praštevil
candidate: DB 2         ; trenutni kandidat
divisor: DB 0           ; delitelj za preverjanje

main:
    MOV A, [N]
    MOV [count], A      ; count = N (štejemo nazaj)
    MOV A, 2
    MOV [candidate], A  ; začnemo pri 2
    
prime_loop:
    MOV A, [count]
    CMP A, 0
    JZ prime_done       ; če smo našli dovolj
    
    ; preveri, če je candidate praštevilo
    MOV A, [candidate]
    CALL is_prime       ; rezultat v B (1=da, 0=ne)
    
    CMP B, 1
    JNZ not_prime
    
    ; je praštevilo - potisni na sklad
    MOV A, [candidate]
    PUSH A
    
    ; zmanjšaj števec
    MOV A, [count]
    DEC A
    MOV [count], A
    
not_prime:
    ; povečaj kandidata
    MOV A, [candidate]
    INC A
    MOV [candidate], A
    JMP prime_loop
    
prime_done:
    HLT

; Funkcija is_prime
; Vhod: A = število za preverjanje
; Izhod: B = 1 če praštevilo, 0 sicer
is_prime:
    PUSH A
    PUSH C
    PUSH D
    
    ; posebni primeri
    CMP A, 2
    JZ is_prime_yes     ; 2 je praštevilo
    
    ; preveri, če je sodo (A % 2 == 0)
    MOV C, A
    MOV D, 2
    CALL mod            ; rezultat v B
    CMP B, 0
    JZ is_prime_no      ; sodo število ni praštevilo (razen 2)
    
    ; preveri delitelje od 3 naprej (samo lihe)
    MOV D, 3            ; D = delitelj
    
check_divisor:
    ; če je D*D > A, je praštevilo
    MOV C, D
    PUSH A
    ; izračunaj D*D z množenjem
    MOV A, 0            ; A = rezultat množenja
    MOV B, D            ; B = števec
mul_loop:
    CMP B, 0
    JZ mul_done
    ADD A, D
    DEC B
    JMP mul_loop
mul_done:
    MOV B, A            ; B = D*D
    POP A               ; A = kandidat
    CMP B, A
    JA is_prime_yes     ; D*D > A -> praštevilo
    
    ; preveri, če D deli A
    MOV C, A            ; C = kandidat
    PUSH D
    CALL mod            ; A mod D -> B
    POP D
    CMP B, 0
    JZ is_prime_no      ; deljivo -> ni praštevilo
    
    ; naslednji lihi delitelj
    ADD D, 2
    JMP check_divisor
    
is_prime_yes:
    MOV B, 1
    JMP is_prime_done
    
is_prime_no:
    MOV B, 0
    
is_prime_done:
    POP D
    POP C
    POP A
    RET

; Funkcija mod (ostanek pri deljenju)
; Vhod: C = deljenec, D = delitelj
; Izhod: B = C mod D
mod:
    MOV B, C
mod_loop:
    CMP B, D
    JB mod_done         ; če je B < D, končaj
    SUB B, D
    JMP mod_loop
mod_done:
    RET
