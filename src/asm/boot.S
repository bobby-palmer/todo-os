# 1) Identity and virtual map 1GB at start of kernel space using SV 39
# 2) Set stack
# 3) jump to kmain

.equ PAGE_SHIFT, 12
.equ PAGE_SIZE, 1 << 12

.section .text.boot
.global _start

_start:
  la t0, _kernel_pt
  la t1, _RAM_START
  srli t1, t1, PAGE_SHIFT /*t1 is PPN of ram start*/

  # Create pte in t2
  slli t2, t1, 10
  addi t2, t2, 0xF

  # identity map
  srli t1, t1, (9 + 9)
  slli t1, t1, 3 /* Multiply by size of pte*/
  add t1, t0, t1 /* Get entry location */
  sd t2, 0(t1)

  # Virtual map
  li t1, 1
  slli t1, t1, 3 + 8
  add t1, t0, t1
  sd t2, 0(t1)

  # start mmu
  srli t0, t0, PAGE_SHIFT
  li t1, 8
  slli t1, t1, 60
  or t0, t0, t1
  csrw satp, t0

  # set sp to boot stack
  la t0, sstack
  ld t0, 0(t0)
  mv sp, t0

  # make sure sscratch is zeroed
  li t0, 0
  csrw sscratch, t0

  # load kmain addr and jump
  la t0, kmain_ptr
  ld t0, 0(t0)
  jalr zero, t0, 0

.align PAGE_SHIFT
_kernel_pt: .zero PAGE_SIZE

sstack: .quad _sstack
kmain_ptr: .quad kmain
