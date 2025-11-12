.global _exception_entry
.section .text.exception_entry "ax"
.align 4

_exception_entry:
  /*TODO stack switch*/
  csrw sscratch, sp
