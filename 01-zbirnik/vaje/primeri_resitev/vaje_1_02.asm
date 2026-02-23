; Task 2: Division by repeated subtraction: quotient in C, remainder in A

JMP main
main:
    MOV A, 27
    MOV B, 5
    MOV C, 0
div_loop:
    CMP A, B
    JB div_done
    SUB A, B
    INC C
    JMP div_loop
div_done:
    ; C = quotient, A = remainder
    HLT
