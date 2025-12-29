.section .text.boot
.global _start

# Constants
.equ VIRT_OFFSET,   0xFFFFFFFF00000000   # _VIRT_BASE - _RAM_START
.equ SATP_SV39,     (8 << 60)
.equ PAGE_SIZE,     4096

# PTE flags
.equ PTE_V, (1 << 0)  # Valid
.equ PTE_R, (1 << 1)  # Read
.equ PTE_W, (1 << 2)  # Write
.equ PTE_X, (1 << 3)  # Execute
.equ PTE_A, (1 << 6)  # Accessed
.equ PTE_D, (1 << 7)  # Dirty

.equ PTE_KERNEL, (PTE_V | PTE_R | PTE_W | PTE_X | PTE_A | PTE_D)

_start:
    # a0 = hartid (from OpenSBI)
    # a1 = pointer to device tree (from OpenSBI)

    # Only let hart 0 proceed, park other harts
    bnez a0, _park

    # Save a0 and a1 for kmain
    mv s0, a0
    mv s1, a1

    # -------------------------------------------------
    # Set up initial page tables for Sv39
    # We need to identity map 0x80000000 region (for current code)
    # and map virtual 0xFFFFFFFF80000000 to physical 0x80000000
    # Using 1GB gigapages for simplicity
    # -------------------------------------------------

    # Get physical address of boot_page_table (it's in .text.boot, so la works)
    la t0, boot_page_table

    # Clear the boot page table
    li t1, PAGE_SIZE
1:
    sd zero, (t0)
    addi t0, t0, 8
    addi t1, t1, -8
    bnez t1, 1b

    # Sv39: 512 entries per level, each covering different ranges
    # VPN[2] (bits 38:30) selects 1GB region
    #
    # Physical 0x80000000 -> VPN[2] = 2
    # Virtual 0xFFFFFFFF80000000 -> VPN[2] = 510
    #
    # Create gigapage PTEs (PPN points directly to physical page)

    la t0, boot_page_table

    # Identity map: entry 2 -> physical 0x80000000
    # PTE = (0x80000000 >> 12) << 10 | flags = 0x20000000 | flags
    li t1, (0x80000000 >> 2) | PTE_KERNEL
    sd t1, (2 * 8)(t0)

    # Higher-half map: entry 510 -> physical 0x80000000
    # Same PTE value, different slot
    li t2, (510 * 8)
    add t2, t0, t2
    sd t1, 0(t2)

    # -------------------------------------------------
    # Enable paging
    # -------------------------------------------------

    # Set satp: MODE=Sv39, ASID=0, PPN=boot_page_table >> 12
    la t0, boot_page_table
    srli t0, t0, 12
    li t1, SATP_SV39
    or t0, t0, t1

    # Ensure previous stores are visible
    sfence.vma zero, zero

    # Enable paging
    csrw satp, t0

    # Flush TLB
    sfence.vma zero, zero

    # -------------------------------------------------
    # Jump to higher half
    # We use absolute addressing since _higher_half_entry is
    # a virtual address defined in linker script space
    # -------------------------------------------------

    # Calculate virtual address of _higher_half label
    # _higher_half is in .text.boot at physical address
    # We need its virtual equivalent: phys + VIRT_OFFSET
    la t0, _higher_half
    li t1, VIRT_OFFSET
    add t0, t0, t1
    jr t0

_higher_half:
    # Now running in higher half virtual addresses

    # -------------------------------------------------
    # Set up stack (use absolute address load)
    # -------------------------------------------------

    # Load the virtual address of _stack_top
    lla t0, _stack_top_addr
    ld sp, 0(t0)

    # -------------------------------------------------
    # Zero BSS (use absolute address load)
    # -------------------------------------------------

    lla t0, _sbss_addr
    ld t0, 0(t0)
    lla t1, _ebss_addr
    ld t1, 0(t1)
1:
    bgeu t0, t1, 2f
    sd zero, (t0)
    addi t0, t0, 8
    j 1b
2:

    # -------------------------------------------------
    # Jump to kmain(hartid, dtb_ptr)
    # -------------------------------------------------

    mv a0, s0
    mv a1, s1

    # Load kmain address from constant pool
    lla t0, _kmain_addr
    ld t0, 0(t0)
    jalr ra, t0, 0

    # kmain should never return, but just in case
    j _park

_park:
    wfi
    j _park


# -------------------------------------------------
# Constant pool for virtual addresses
# These are in .text.boot so they're reachable with PC-relative addressing
# -------------------------------------------------
.balign 8
_stack_top_addr:
    .quad _stack_top
_sbss_addr:
    .quad _sbss
_ebss_addr:
    .quad _ebss
_kmain_addr:
    .quad kmain

# -------------------------------------------------
# Boot page table - must be page aligned (4KB)
# Keep in .text.boot so it's at physical address during boot
# -------------------------------------------------
.balign 4096
boot_page_table:
    .skip 4096
