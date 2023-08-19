OUTPUT_FORMAT(elf32-i386)
OUTPUT_ARCH(i386)
ENTRY (_start)
SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}