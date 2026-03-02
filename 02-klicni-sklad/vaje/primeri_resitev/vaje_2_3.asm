; Vaja 2.3: Menjava pomnilniških celic
; swap_mem: zamenja vsebini [A] in [B]
; Ohrani vse registre

JMP main

; Testni podatki
x: DB 42
y: DB 17

main:
    MOV A, x            ; A = naslov x
    MOV B, y            ; B = naslov y
    CALL swap_mem
    ; Po klicu: [x] = 17, [y] = 42
    HLT

; Funkcija swap_mem
; Vhod: A = naslov1, B = naslov2
; Zamenja [A] in [B]
swap_mem:
    PUSH C
    PUSH D
    MOV C, [A]          ; C = vrednost na [A]
    MOV D, [B]          ; D = vrednost na [B]
    MOV [A], D          ; [A] = stara vrednost [B]
    MOV [B], C          ; [B] = stara vrednost [A]
    POP D
    POP C
    RET
