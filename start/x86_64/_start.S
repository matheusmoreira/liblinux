#include <liblinux/definitions.h>

.text
.global _start

_start:
	xorq %rbp,%rbp    /* user code should mark the deepest stack frame
	                   * by setting the frame pointer to zero
	                   */

	movq %rsp,%rdi

	call liblinux_start

	movq %rax,%rdi
	movq $__NR_exit,%rax
	syscall

	hlt
