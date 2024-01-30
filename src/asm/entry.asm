extern kmain
extern DUPA22

global start

section .text
bits 32
start:
    ; save the initial state and pass it to kmain
    mov edi, eax
    mov esi, ebx

    ; Point the first entry of the level 4 page table to the first entry in the
    ; p3 table
    mov eax, p3_table
    or eax, 0b11
    mov dword [p4_table + 0], eax

    ; point each page table level two entry to a page
    mov dword [p3_table + 0], 0b10000011
    mov dword [p3_table + 8], 0b10000011 | 0x40000000
    mov dword [p3_table + 16], 0b10000011 | 0x80000000
    mov dword [p3_table + 24], 0b10000011 | 0xC0000000


    ; move page table address to cr3
    mov eax, p4_table
    mov cr3, eax

    ; enable PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; set the long mode bit
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; enable paging
    mov eax, cr0
    or eax, 1 << 31
    or eax, 1 << 16
    mov cr0, eax

    ; load GDT
    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    ; enter long mode
    jmp gdt64.code:kmain

debug:
    mov dx, 0x3F8 ; COM1

    mov al, 0x48  ; H
    out dx, al
    mov al, 0x45  ; E
    out dx, al
    mov al, 0x52  ; R
    out dx, al
    mov al, 0x45  ; E
    out dx, al
    mov al, 0x0A  ; \n
    out dx, al
    mov al, 0x0D  ; \r
    out dx, al
    ret


section .bss

align 4096

p4_table:
    resb 4096
p3_table:
    resb 4096


section .rodata
; dummy gdt for transitioning to long mode
gdt64:
    dq 0
.code: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41) | (1<<43) | (1<<53)
.data: equ $ - gdt64
    dq (1<<44) | (1<<47) | (1<<41)

.pointer:
    dw .pointer - gdt64 - 1
    dq gdt64


