; Vaja 2.2: Funkcija poisci_minimum
; A = začetek rezine, C = konec rezine (naslednji naslov za zadnjim)
; Vrne v B indeks najmanjšega elementa (0..n-1)
; Ohrani vse registre razen B

JMP main

; Testni podatki
len: DB 6
data: DB 45
      DB 12
      DB 78
      DB 5
      DB 34
      DB 89
result: DB 0
min_val: DB 0

main:
    MOV A, data         ; začetek rezine
    MOV C, data
    ADD C, [len]        ; konec rezine (data + [len])
    CALL poisci_minimum
    MOV [result], B     ; shrani indeks (mora biti 3, ker je 5 na indeksu 3)
    HLT

; Funkcija: poisci_minimum
; Vhod: A = start, C = end
; Izhod: B = indeks minimuma
; Ohrani: A, C, D
poisci_minimum:
    PUSH A
    PUSH C
    PUSH D
    
    ; inicializacija
    MOV B, 0            ; min_index = 0
    MOV D, [A]          ; D = prvi element
    MOV [min_val], D    ; shrani kot trenutni minimum
    
    MOV D, 1            ; D = tekoci indeks (zacnemo pri 1)
    INC A               ; A kaze na drugi element
    
pm_loop:
    CMP A, C            ; ce smo na koncu
    JE pm_done
    
    ; primerjaj [A] z min_val
    PUSH D              ; shrani indeks
    MOV D, [A]          ; D = trenutni element
    CMP D, [min_val]
    JNB pm_skip         ; ce ni manjsi, preskoci
    
    ; nov minimum
    MOV [min_val], D    ; shrani nov minimum
    POP D               ; D = trenutni indeks
    MOV B, D            ; B = nov min_index
    JMP pm_next
    
pm_skip:
    POP D               ; obnovi indeks
    
pm_next:
    INC A               ; naslednji element
    INC D               ; povecaj indeks
    JMP pm_loop
    
pm_done:
    POP D
    POP C
    POP A
    RET

