.section .text
.global _trapentry

.align 4
_trapentry:
  li a0, 0xd00d
  wfi
