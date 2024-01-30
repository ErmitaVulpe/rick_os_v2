section .multiboot_header
header_start:
    dd 0xe85250d6                ; magic number
    dd 0                         ; protected mode code
    dd header_end - header_start ; header length

    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; request video mode
    dw 5        ; magic number
    dw 0        ; flags
    dd 20       ; size of the tag
    dd 1024     ; requested width
    dd 768      ; requested height
    dd 32       ; BPP (bits per pixel)
header_end:
