; Task 8: leave the maximum of A and B in A

JMP main
main:
    MOV A, 4
    MOV B, 9
    CMP A, B
    JNB done_max
    MOV A, B
done_max:
    HLT
