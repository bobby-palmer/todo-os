OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Physical RAM starts at 0x80000000, OpenSBI loads kernel at 0x80200000 */
RAM_START = 0x80000000;
KERN_LOAD = 0x80200000;

/* Virtual base for higher half (1GiB gigapage maps 0xFFFFFFFF80000000 -> 0x80000000) */
VIRT_BASE = 0xFFFFFFFF80000000;

/* Offset: virtual = physical + KERN_OFFSET */
KERN_OFFSET = VIRT_BASE - RAM_START;

/* Stack size: 64 KiB */
STACK_SIZE = 0x10000;

SECTIONS
{
    /* Boot code at physical load address (runs with MMU off) */
    . = KERN_LOAD;

    .text.boot : {
        *(.text.boot)
    }

    .data.boot : {
        *(.data.boot)
    }

    /* Page table (4K aligned) */
    . = ALIGN(4K);
    boot_page_table_l2 = .;
    . += 4K;

    /* End of boot section */
    _boot_end = .;

    /*
     * Switch to virtual addresses for the kernel.
     * Gigapage maps virtual 0xFFFFFFFF80000000 -> physical 0x80000000.
     */
    . = ALIGN(4K);
    . = . + KERN_OFFSET;

    .text : AT(ADDR(.text) - KERN_OFFSET) {
        _text_start = .;
        *(.text .text.*)
        _text_end = .;
    }

    . = ALIGN(4K);

    .rodata : AT(ADDR(.rodata) - KERN_OFFSET) {
        _rodata_start = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        _rodata_end = .;
    }

    . = ALIGN(4K);

    .data : AT(ADDR(.data) - KERN_OFFSET) {
        _data_start = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        _data_end = .;
    }

    . = ALIGN(4K);

    .bss : AT(ADDR(.bss) - KERN_OFFSET) {
        _bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        _bss_end = .;
    }

    . = ALIGN(4K);

    /* Kernel stack */
    _stack_bottom = .;
    . += STACK_SIZE;
    _stack_top = .;

    _kernel_end = .;

    /* Export symbols */
    PROVIDE(__ram_start = RAM_START);
    PROVIDE(__virt_base = VIRT_BASE);
    PROVIDE(__kern_offset = KERN_OFFSET);

    /DISCARD/ : {
        *(.eh_frame)
    }
}
