ENTRY(_start)

_RAM_START    = 0x80000000;
_KERNEL_START = 0x80200000;

SECTIONS {

  . = _KERNEL_START;

  .text.boot : {
    *(.text.boot);
  }

  .text : {
    *(text .text.*);
  }

  .rodata ALIGN(8) :  {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);
    . = ALIGN(8);
  }

  .data ALIGN(8) :  {
    *(.data .data.*);
  }

  .bss ALIGN(64) (NOLOAD) : {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(64);
    _ebss = .;
  }
}
