.global _exception_entry
.section .text.exception_entry "ax"
.align 4

_exception_entry:
  # switch to exception stack
  csrrw sp, sp sscratch

  # Alloc stack space for frame
  addi sp, sp, -8 * 31

  # Save all registers except sp
  sd x1, 8 * 0(sp)
  # sp == x2
  sd x3, 8 * 2(sp)
  sd x4, 8 * 3(sp)
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

  # get and save sp
  csrr a0, sscratch
  sd a0, 8 * 1(sp)

  # set argument to point to trap frame and jump to handler
  mov a0, sp
  call handle_trap

  # restore registers from frame (stack after)
  ld x1, 8 * 0(sp)
  # sp == x2
  ld x3, 8 * 2(sp)
  ld x4, 8 * 3(sp)
  ld x5, 8 * 4(sp)
  ld x6, 8 * 5(sp)
  ld x7, 8 * 6(sp)
  ld x8, 8 * 7(sp)
  ld x9, 8 * 8(sp)
  ld x10, 8 * 9(sp)
  ld x11, 8 * 10(sp)
  ld x12, 8 * 11(sp)
  ld x13, 8 * 12(sp)
  ld x14, 8 * 13(sp)
  ld x15, 8 * 14(sp)
  ld x16, 8 * 15(sp)
  ld x17, 8 * 16(sp)
  ld x18, 8 * 17(sp)
  ld x19, 8 * 18(sp)
  ld x20, 8 * 19(sp)
  ld x21, 8 * 20(sp)
  ld x22, 8 * 21(sp)
  ld x23, 8 * 22(sp)
  ld x24, 8 * 23(sp)
  ld x25, 8 * 24(sp)
  ld x26, 8 * 25(sp)
  ld x27, 8 * 26(sp)
  ld x28, 8 * 27(sp)
  ld x29, 8 * 28(sp)
  ld x30, 8 * 29(sp)
  ld x31, 8 * 30(sp)

  # restore stack pointer
  ld sp, 8 * 1(sp)

  # finish
  sret
