# TrapFrame stuct size
.equ FRAME_SIZE, -8 * 32

# Save registers other than sp and tp to the stack
.macro PUSH_REGISTERS
  addi sp, sp, FRAME_SIZE
  sd x1, 8 * 0(sp)
  # sp = x2
  sd x3, 8 * 2(sp)
  # tp = x4
  sd x5, 8 * 4(sp)
  sd x6, 8 * 5(sp)
  sd x7, 8 * 6(sp)
  sd x8, 8 * 7(sp)
  sd x9, 8 * 8(sp)
  sd x10, 8 * 9(sp)
  sd x11, 8 * 10(sp)
  sd x12, 8 * 11(sp)
  sd x13, 8 * 12(sp)
  sd x14, 8 * 13(sp)
  sd x15, 8 * 14(sp)
  sd x16, 8 * 15(sp)
  sd x17, 8 * 16(sp)
  sd x18, 8 * 17(sp)
  sd x19, 8 * 18(sp)
  sd x20, 8 * 19(sp)
  sd x21, 8 * 20(sp)
  sd x22, 8 * 21(sp)
  sd x23, 8 * 22(sp)
  sd x24, 8 * 23(sp)
  sd x25, 8 * 24(sp)
  sd x26, 8 * 25(sp)
  sd x27, 8 * 26(sp)
  sd x28, 8 * 27(sp)
  sd x29, 8 * 28(sp)
  sd x30, 8 * 29(sp)
  sd x31, 8 * 30(sp)
.endm

.section .text
.global _trap_entry
.align 4

_trap_entry:
  wfi
  csrrw tp, sscratch, tp
  beqz tp, early_kernel

other_times:
  li a0, 0xB00B5
  wfi

early_kernel:
  PUSH_REGISTERS 

  addi t1, sp, FRAME_SIZE
  sd   t1, 8 * 1(sp)

  csrrw t1, sscratch, tp
  sd    t1, 8 * 3(sp)

  mv a0, sp
  call handle_trap

L:
  j L
