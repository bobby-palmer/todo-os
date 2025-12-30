# Boot entry point for todo-os
# RISC-V 64-bit assembly
# Sets up Sv39 paging and jumps to higher half kernel

.section .text.boot
.global _start

# Constants
.equ SATP_SV39,     (8 << 60)
.equ PAGE_SIZE,     4096
.equ PTE_V,         (1 << 0)    # Valid
.equ PTE_R,         (1 << 1)    # Readable
.equ PTE_W,         (1 << 2)    # Writable
.equ PTE_X,         (1 << 3)    # Executable
.equ PTE_G,         (1 << 5)    # Global
.equ PTE_A,         (1 << 6)    # Accessed
.equ PTE_D,         (1 << 7)    # Dirty

# Kernel mapping flags (RWX, Global, Accessed, Dirty)
.equ PTE_KERN,      (PTE_V | PTE_R | PTE_W | PTE_X | PTE_G | PTE_A | PTE_D)

# Address constants (must match linker.ld)
.equ RAM_START,     0x80000000

_start:
    # a0 = hart id, a1 = device tree pointer (from OpenSBI)

    # Only let hart 0 continue, park other harts
    bnez a0, .Lpark

    # =========================================
    # Setup Sv39 page tables
    # =========================================
    # We need to map:
    # 1. Identity map: 0x80000000 -> 0x80000000 (for boot code after MMU on)
    # 2. Higher half: 0xFFFFFFFF80000000 -> 0x80000000 (for kernel)
    # Using 1GiB gigapages for simplicity

    # Clear page table
    la t0, boot_page_table_l2
    li t1, PAGE_SIZE
    add t1, t0, t1
1:
    sd zero, (t0)
    addi t0, t0, 8
    bltu t0, t1, 1b

    # Build PTE for 0x80000000 physical (1GiB gigapage)
    # PTE = (RAM_START >> 12) << 10 | PTE_KERN
    li t1, RAM_START
    srli t1, t1, 12         # Get PPN
    slli t1, t1, 10         # Shift to PTE position
    li t2, PTE_KERN
    or t1, t1, t2           # Add flags

    la t0, boot_page_table_l2

    # Identity map: L2[2] -> 0x80000000
    # VPN[2] for 0x80000000 = (0x80000000 >> 30) & 0x1FF = 2
    sd t1, 16(t0)           # 2 * 8 = 16

    # Higher half: L2[510] -> 0x80000000
    # VPN[2] for 0xFFFFFFFF80000000 = 510
    li t2, 4080             # 510 * 8 = 4080
    add t2, t0, t2
    sd t1, 0(t2)

    # =========================================
    # Enable Sv39 paging
    # =========================================
    # satp = SATP_SV39 | (boot_page_table_l2 >> 12)
    la t0, boot_page_table_l2
    srli t0, t0, 12         # Get PPN of page table
    li t1, SATP_SV39
    or t0, t0, t1

    sfence.vma zero, zero
    csrw satp, t0
    sfence.vma zero, zero

    # =========================================
    # Jump to higher half
    # =========================================
    la t0, .Ltrampoline_addr
    ld t0, 0(t0)
    jr t0

.Lpark:
    wfi
    j .Lpark

# Store the virtual address of the trampoline
.section .data.boot
.align 3
.Ltrampoline_addr:
    .dword higher_half_trampoline

# =========================================
# Higher half trampoline (linked at virtual addresses)
# =========================================
.section .text
.global higher_half_trampoline

higher_half_trampoline:
    # Now running at higher half virtual addresses

    # Zero BSS section
    la t0, _bss_start
    la t1, _bss_end
1:
    bgeu t0, t1, 2f
    sd zero, (t0)
    addi t0, t0, 8
    j 1b
2:

    # Setup kernel stack
    la sp, _stack_top
    mv fp, zero

    # Jump to Rust kmain (a0 = hart id, a1 = dtb pointer)
    call kmain

    # kmain should never return
1:
    wfi
    j 1b
