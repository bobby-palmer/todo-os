# Boot entry point for todo-os
# RISC-V 64-bit assembly
# Runs before MMU is enabled at physical addresses

.section .text.boot
.global _start

_start:
    # Entry point - runs at physical address before MMU setup
    # TODO: Setup page tables and enable MMU
    # TODO: Jump to higher half kernel

1:
    wfi                 # Wait for interrupt
    j 1b                # Loop forever
