section .multiboot_header

header_start:
    dd 0xe85250d6                ; magic number (multiboot 2)
    dd 0                         ; architecture 0 (protected mode i386)
    dd header_end - header_start ; header length
    ; checksum
    dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    ; insert optional multiboot tags here

    ; required end tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
header_end:

section .text

global _start

global stack_bottom
global stack_top

extern main

_start:
    mov esp, stack_top
    call main
    cli
    hlt

section .bss
stack_bottom:
    resb 0x5000
stack_top:
