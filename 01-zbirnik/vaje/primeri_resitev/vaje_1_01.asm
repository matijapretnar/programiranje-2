; Task 1: remainder A % B via repeated subtraction

JMP main
main:
    MOV A, 37    ; dividend
    MOV B, 5     ; divisor
rem_loop:
    CMP A, B
    JB rem_done
    SUB A, B
    JMP rem_loop
rem_done:
    ; A contains remainder
    HLT
