; Vaja 2.4: Selection sort - MODULARNA REŠITEV
; Sestavljeno iz funkcij nalog 1, 2, 3:
;   - poisci_minimum (vaja 2.2)
;   - swap_mem (vaja 2.3)
;
; Algoritem:
;   dokler nisi na koncu seznama:
;     1. poišči minimum od trenutne pozicije do konca
;     2. zamenjaj ga s trenutnim mestom
;     3. premakni se na naslednjo pozicijo

JMP main

; Testni podatki
data: DB 45
      DB 12
      DB 78
      DB 5
      DB 34
      DB 89
len: DB 6
min_val: DB 0           ; pomožna spremenljivka za poisci_minimum

main:
    MOV A, data
    MOV C, data
    ADD C, [len]        ; C = konec rezine
    CALL uredi
    HLT

; ============================================
; Funkcija: uredi (selection sort)
; Vhod: A = začetek, C = konec
; Uredi elemente od najmanjšega do največjega
; Sestavi se iz klicev poisci_minimum in swap_mem
; ============================================
uredi:
    PUSH A
    PUSH B
    PUSH C
    PUSH D
    
    MOV D, A            ; D = kazalec na trenutno pozicijo
    
uredi_loop:
    ; preveri ali smo končali (D >= C-1)
    PUSH A
    MOV A, C
    DEC A               ; A = C - 1
    CMP D, A
    POP A
    JNB uredi_done      ; če D >= C-1, končaj
    
    ; ---- KORAK 1: Poišči minimum v [D, C) ----
    PUSH A
    PUSH C
    MOV A, D            ; začetek iskanja = trenutna pozicija
    ; C ostane konec
    CALL poisci_minimum ; vrne indeks minimuma v B
    
    ; izračunaj naslov minimuma: D + B
    PUSH D
    ADD D, B            ; D = naslov minimuma
    MOV B, D            ; B = naslov minimuma (za swap_mem)
    POP D
    POP C
    POP A
    
    ; ---- KORAK 2: Zamenjaj [D] z minimumom ----
    ; A = D (trenutna pozicija), B = naslov minimuma
    PUSH A
    MOV A, D            ; A = naslov trenutne pozicije
    CMP A, B
    JE uredi_skip_swap  ; če A == B, ne rabimo zamenjati
    CALL swap_mem       ; zamenjaj [A] in [B]
    
uredi_skip_swap:
    POP A
    
    ; ---- KORAK 3: Premakni se na naslednjo pozicijo ----
    INC D
    JMP uredi_loop
    
uredi_done:
    POP D
    POP C
    POP B
    POP A
    RET

; ============================================
; Funkcija: poisci_minimum (iz vaje 2.2)
; Vhod: A = začetek rezine, C = konec rezine
; Izhod: B = indeks najmanjšega elementa (0..n-1)
; Ohrani: A, C, D
; ============================================
poisci_minimum:
    PUSH A
    PUSH C
    PUSH D
    
    ; inicializacija
    MOV B, 0            ; min_index = 0
    MOV D, [A]          ; D = prvi element
    MOV [min_val], D    ; shrani kot trenutni minimum
    
    MOV D, 1            ; D = tekoči indeks (začnemo pri 1)
    INC A               ; A kaže na drugi element
    
pm_loop:
    CMP A, C            ; če smo na koncu
    JE pm_done
    
    ; primerjaj [A] z min_val
    PUSH D              ; shrani indeks
    MOV D, [A]          ; D = trenutni element
    CMP D, [min_val]
    JNB pm_skip         ; če ni manjši, preskoči
    
    ; nov minimum
    MOV [min_val], D    ; shrani nov minimum
    POP D               ; D = trenutni indeks
    MOV B, D            ; B = nov min_index
    JMP pm_next
    
pm_skip:
    POP D               ; obnovi indeks
    
pm_next:
    INC A               ; naslednji element
    INC D               ; povečaj indeks
    JMP pm_loop
    
pm_done:
    POP D
    POP C
    POP A
    RET

; ============================================
; Funkcija: swap_mem (iz vaje 2.3)
; Vhod: A = naslov1, B = naslov2
; Zamenja vsebini [A] in [B]
; Ohrani: vse registre
; ============================================
swap_mem:
    PUSH C
    PUSH D
    MOV C, [A]          ; C = vrednost na [A]
    MOV D, [B]          ; D = vrednost na [B]
    MOV [A], D          ; [A] = stara vrednost [B]
    MOV [B], C          ; [B] = stara vrednost [A]
    POP D
    POP C
    RET

