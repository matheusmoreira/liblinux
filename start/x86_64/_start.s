.text
.global _start

_start:
	movq %rsp,%rdi

	call liblinux_start

	movq %rax,%rdi
	movq $60,%rax
	syscall

	hlt
