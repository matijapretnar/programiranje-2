; Task 6: iterative Fibonacci (B = n -> A = F(n))

JMP main
input: DB 8
tmp: DB 0

main:
    MOV B, [input]   ; B = n
    MOV A, 0         ; A = F(0)
    MOV C, 1         ; C = F(1)
    MOV D, 0         ; counter i = 0
fib_loop:
    CMP D, B
    JNB fib_done     ; if i >= n -> done
    MOV [tmp], C     ; tmp = current C (old F(i+1))
    ADD C, A         ; C = oldC + oldA -> next
    MOV A, [tmp]     ; A = oldC
    INC D
    JMP fib_loop
fib_done:
    HLT
