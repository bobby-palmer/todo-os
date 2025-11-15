.equ PAGE_SHIFT, 12
.equ PAGE_SIZE, 1 << PAGE_SHIFT  /* 4096 bytes */
.equ FLAGS, 0xF

.section .text.boot
.global _start

_start:
  la t0, _kernel_pt
  li t1, FLAGS /*Pte entry mapping to phys addr 0*/
  
  # identity map
  sd t1, 0(t0)

  # higher half identity map
  li t2, 0x100
  slli t2, t2, 3
  add t2, t0, t2
  sd t1, 0(t2)

  # start mmu
  srli t0, t0, PAGE_SHIFT
  li t1, 9
  slli t1, t1, 60
  or t0, t0, t1
  csrw satp, t0

  # set sp to boot stack
  la t0, sstack
  ld t0, 0(t0)
  mv sp, t0

  # load kmain addr and jump
  la t0, kmain_ptr
  ld t0, 0(t0)
  jalr zero, t0, 0

.align PAGE_SHIFT
_kernel_pt: .zero PAGE_SIZE

sstack: .quad _sstack
kmain_ptr: .quad kmain
