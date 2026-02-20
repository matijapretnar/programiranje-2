; Task 4: swap contents of memory cells [A] and [B] using register

JMP main
input_A: DB 11
input_B: DB 22


main:
    MOV A, [input_A]
    MOV B, [input_B]
    MOV C, A
    MOV A, B
    MOV B, C
    MOV [input_A], A
    MOV [input_B], B
