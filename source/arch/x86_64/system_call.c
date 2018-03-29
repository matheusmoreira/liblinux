long
system_call(long number, long _1, long _2, long _3, long _4, long _5, long _6)
{
	long return_value;

	register long r10 __asm__("r10") = _4;
	register long r8  __asm__("r8")  = _5;
	register long r9  __asm__("r9")  = _6;

	__asm__ volatile
	("syscall"

		: "=a" (return_value)
		: "a" (number),
		  "D" (_1), "S" (_2), "d" (_3),
		  "r" (r10), "r" (r8), "r" (r9)
		: "rcx", "r11", "cc", "memory");

	return return_value;
}
