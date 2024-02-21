extern kmain
extern map_memory
extern p4_table

global _start
global print
global debug


section .text
bits 32
_start:
    cmp eax, 0x36d76289
    jne multiboot_not_compliant

    ; save the initial state and pass it to kmain
    push ebx
    mov edi, ebx
    call map_memory

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

    pop edi

    ; load GDT
    lgdt [gdt64.pointer]

    ; update selectors
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax

    ; enter long mode
    jmp gdt64.code:kmain


; prints a string contained where the porter in esi points, has to be null terminated
print:
    push dx
    push ax

    mov dx, 0x3F8   ; COM1
    cld             ; clear the direction flag (increment)

    print_loop:
    lodsb           ; progress the esi
    test al, al     ; check for null terminator
    jz end_print
    out dx, al
    jmp print_loop

    end_print:
    mov al, 0x0A ; \n
    out dx, al
    mov al, 0x0D ; \r
    out dx, al
    pop ax
    pop dx
    ret

debug:
    push esi
    mov esi, msg_debug
    call print
    pop esi
    ret

global multiboot_not_compliant
multiboot_not_compliant:
    mov esi, msg_multiboot_not_compliant
    call print
    hlt

global unsupported_multiboot2_version
unsupported_multiboot2_version:
    mov esi, msg_multiboot_version_not_supported
    call print
    hlt

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


msg_multiboot_not_compliant db "Bootloader is not multiboot2 compliant", 0
msg_multiboot_version_not_supported db "Unsupported version of the multiboot2 standard", 0
msg_debug db "debug", 0

