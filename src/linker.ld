OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Physical address where kernel is loaded */
PHYS_BASE = 0x80200000;

/* Virtual address for higher half kernel (top 2GiB for -mcmodel=kernel) */
VIRT_BASE = 0xFFFFFFFF80000000;

/* Offset between virtual and physical addresses */
KERN_OFFSET = VIRT_BASE - PHYS_BASE;

SECTIONS
{
    /* Start at physical address for initial boot code */
    . = PHYS_BASE;

    /* Boot code runs with MMU off, must be at physical address */
    .text.boot : {
        *(.text.entry)
        *(.text.boot)
    }

    /* Switch to virtual addresses for the rest of the kernel */
    . += KERN_OFFSET;

    .text : AT(ADDR(.text) - KERN_OFFSET) {
        *(.text .text.*)
    }

    .rodata : AT(ADDR(.rodata) - KERN_OFFSET) {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data : AT(ADDR(.data) - KERN_OFFSET) {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : AT(ADDR(.bss) - KERN_OFFSET) {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }

    /* Export symbols for use in kernel code */
    PROVIDE(__phys_base = PHYS_BASE);
    PROVIDE(__virt_base = VIRT_BASE);
    PROVIDE(__kern_offset = KERN_OFFSET);

    /DISCARD/ : {
        *(.eh_frame)
    }
}
