; Task 3: write numbers 13..42 into consecutive memory starting at label out

JMP main
main:
    MOV A, 13
    MOV B, out ; try to start on 3
seq_loop:
    MOV [B], A
    INC A
    INC B
    CMP A, 43
    JB seq_loop
    HLT

; data block placed after code to reserve space
out: 