global map_memory
global p4_table

section .text
bits 32

; takes the ptr to the mbi in the edi register
map_memory:
    push eax
    push ebx
    push ecx
    push edx

    ; push edi

    ; insert the p3_identity to the 0th entry of p4_table
    mov eax, p3_identity
    or eax, 0b11
    mov dword [p4_table + 0], eax
    ; mov eax, p3_offset
    or eax, 0b11
    mov dword [p4_table + 256], eax

    ; identity map
    mov ecx, 128            ; num iterations
    xor eax, eax            ; eax will hold most significant 32 bits of page addresses
    mov edi, p3_identity    ; pointer to the p3 table

    populate_p3_identity:
    mov dword [edi], 0b10000011 | 0b10000
    mov dword [edi + 4], eax
    add edi, 8
    mov dword [edi], 0x40000000 | 0b10000011 | 0b10000
    mov dword [edi + 4], eax
    add edi, 8
    mov dword [edi], 0x80000000 | 0b10000011 | 0b10000
    mov dword [edi + 4], eax
    add edi, 8
    mov dword [edi], 0xC0000000 | 0b10000011 | 0b10000
    mov dword [edi + 4], eax
    add edi, 8
    add eax, 1
    loop populate_p3_identity


    pop edx
    pop ecx
    pop ebx
    pop eax
    ret

section .bss

align 4096

p4_table:
    resb 4096
global p3_identity
p3_identity:
    resb 4096
