; Vaja 2.6: Obrni seznam v pomnilniku
; Obrne seznam dolžine N z uporabo dveh kazalcev

JMP main

; Testni podatki
data: DB 1
      DB 2
      DB 3
      DB 4
      DB 5
      DB 6
len: DB 6

main:
    ; Nastavi kazalce
    MOV A, data         ; A = levi kazalec (začetek)
    MOV B, data
    ADD B, [len]
    DEC B               ; B = desni kazalec (konec - 1)
    
    CALL obrni
    HLT

; Funkcija obrni
; A = levi kazalec, B = desni kazalec
; Obrne elemente med A in B (vključno)
obrni:
    PUSH A
    PUSH B
    PUSH C
    PUSH D
    
obrni_loop:
    ; Če A >= B, končaj
    CMP A, B
    JNB obrni_done
    
    ; Zamenjaj [A] in [B]
    MOV C, [A]
    MOV D, [B]
    MOV [A], D
    MOV [B], C
    
    ; Premakni kazalca
    INC A               ; levi naprej
    DEC B               ; desni nazaj
    JMP obrni_loop
    
obrni_done:
    POP D
    POP C
    POP B
    POP A
    RET
