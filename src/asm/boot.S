# Boot entry point for todo-os
# RISC-V 64-bit assembly

.section .text
.global _start

_start:
    # Entry point - just loop forever for now
    # Disable interrupts (clear MIE bit in mstatus)
1:
    wfi                 # Wait for interrupt
    j 1b                # Loop forever
