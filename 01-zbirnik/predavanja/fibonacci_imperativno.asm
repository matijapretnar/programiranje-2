MOV C, 10
MOV A, 0
MOV B, 1

.loop:
    MOV D, B
    ADD B, A
    MOV A, D
    DEC C
    CMP C, 0
    JNE .loop
HLT