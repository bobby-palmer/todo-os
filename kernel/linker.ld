ENTRY(_start)

_RAM_START    = 0x80000000;
_KERNEL_START = 0x80200000;

_VIRT_BASE    = 0xFFFFFFFF80000000;
_VIRT_OFFSET  = _VIRT_BASE - _RAM_START;

_STACK_LEN = 2M;

SECTIONS {

  . = _KERNEL_START;

  .text.boot : {
    *(.text.boot);
  }

  . = _VIRT_BASE + (_KERNEL_START - _RAM_START) + SIZEOF(.text.boot);

  .text : AT(ADDR(.text) - _VIRT_OFFSET) {
    *(text .text.*);
  }

  .rodata ALIGN(8) : AT(ADDR(.rodata) - _VIRT_OFFSET) {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);
    . = ALIGN(8);
  }

  .data ALIGN(8) : AT(ADDR(.data) - _VIRT_OFFSET) {
    *(.data .data.*);
  }

  .bss ALIGN(64) (NOLOAD) : {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(64);
    _ebss = .;
  }

  .stack ALIGN(16) (NOLOAD) : {
    . += _STACK_LEN;
    _stack_top = .;
  }

  _kend = .;

}
