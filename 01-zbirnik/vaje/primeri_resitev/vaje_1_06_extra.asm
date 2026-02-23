; Task 6: Challange
; Compute F(15) and store it as a 16-bit value (low byte, high byte)
; Uses memory pairs for 16-bit arithmetic (little-endian)

JMP main

; data (16-bit values stored as low,high)
cur_hi: DB 0
cur_lo: DB 0
next_hi: DB 0
next_lo: DB 1
sum_hi: DB 0
sum_lo: DB 0
carry: DB 0
n: DB 15            ; compute F(15)
out_hi: DB 0
out_lo: DB 0

main:
    MOV D, 0        ; counter i = 0
loop_start:
    ; sum_lo = cur_lo + next_lo
    MOV A, [cur_lo]
    MOV B, [next_lo]
    MOV C, A        ; keep original cur_lo in C
    ADD A, B        ; A = sum_lo
    MOV [sum_lo], A
    CMP A, C
    JB set_carry    ; if sum < cur_lo -> carry
    MOV [carry], 0
    JMP carry_done
set_carry:
    MOV [carry], 1
carry_done:

    ; sum_hi = cur_hi + next_hi + carry
    MOV A, [cur_hi]
    MOV B, [next_hi]
    ADD A, B        ; A = cur_hi + next_hi
    MOV B, [carry]
    ADD A, B        ; add carry
    MOV [sum_hi], A

    ; cur = next
    MOV A, [next_lo]
    MOV [cur_lo], A
    MOV A, [next_hi]
    MOV [cur_hi], A

    ; next = sum
    MOV A, [sum_lo]
    MOV [next_lo], A
    MOV A, [sum_hi]
    MOV [next_hi], A

    INC D
    MOV A, D
    CMP A, [n]
    JB loop_start

    ; store result (cur) into out (low, high)
    MOV A, [cur_lo]
    MOV [out_lo], A
    MOV A, [cur_hi]
    MOV [out_hi], A

    HLT
