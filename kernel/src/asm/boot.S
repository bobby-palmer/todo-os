.section .text.boot
.global _start

_start:
  li a0, 0xd00dfeed
L:
  j L
