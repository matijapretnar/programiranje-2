JMP main

; Funkcija, ki izračuna fib([SP]) in vrednost shrani v [SP+1]
fib:
    ; Shranimo vrednosti delovnih registrov
    PUSH A
    PUSH B
    PUSH C

    ; V C shranimo začetno vrednost argumenta
    MOV C, [SP+5]

    ; Če je C = 0, je to tudi rezultat
    CMP C, 0
    JE .fib_end

    ; Če je C = 1, je to tudi rezultat
    CMP C, 1
    JE .fib_end

    ; V nasprotnem primeru najprej izračunamo fib(C - 1) in ga shranimo v B
    DEC C
    PUSH 0  ; prostor za odgovor
    PUSH C
    CALL fib
    POP C
    POP B

    ; Nato izračunamo še fib(C - 2) in ga shranimo v A
    DEC C
    PUSH 0  ; prostor za odgovor
    PUSH C
    CALL fib
    POP C
    POP A
    
    ; Nazadnje k A prištejemo še B, s čimer dobimo končni rezultat
    ADD A, B
    MOV [SP+6], A
    JMP .epilog

.fib_end:
    MOV [SP+6], C

.epilog:
    ; Povrnemo vrednosti registrov in vrnemo rezultat
    POP C
    POP B
    POP A
    RET

main:
    MOV A, 11
    MOV B, 12
    MOV C, 13
    MOV D, 14
    PUSH 0
    PUSH 10
    CALL fib
