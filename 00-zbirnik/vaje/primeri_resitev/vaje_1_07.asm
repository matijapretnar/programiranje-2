; Task 7: XOR swap A and B (no temporary register)

JMP main
main:
    MOV A, 7
    MOV B, 11
    XOR A, B
    XOR B, A
    XOR A, B
