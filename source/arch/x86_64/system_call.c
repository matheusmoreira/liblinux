long
system_call(long number, long _1, long _2, long _3, long _4, long _5, long _6)
{
	long return_value;

	register long rax __asm__("rax") = number;
	register long rdi __asm__("rdi") = _1;
	register long rsi __asm__("rsi") = _2;
	register long rdx __asm__("rdx") = _3;
	register long r10 __asm__("r10") = _4;
	register long r8  __asm__("r8")  = _5;
	register long r9  __asm__("r9")  = _6;

	/* r8, r9 and r10 may be clobbered but can't be in the clobbers list
	   because the compiler won't use clobbered registers as inputs.
	   So they're placed in the outputs list instead. */
	__asm__ volatile
	("syscall"

		: "=a" (return_value),
		  "=r" (r8), "=r" (r9), "=r" (r10)
		: "r" (rax),
		  "r" (rdi), "r" (rsi), "r" (rdx),
		  "r" (r10), "r" (r8),  "r" (r9)
		: "rcx", "r11", "cc", "memory");

	return return_value;
}
