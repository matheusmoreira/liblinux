#include <liblinux/system_calls/exit.h>
#include <liblinux/main.h>

void _start(void)
{
	register long count;
	register char **arguments;
	register char **environment;

	__asm__("movq	0(%%rsp),%0" : "=r" (count));
	__asm__("leaq	8(%%rsp),%0" : "=r" (arguments));

	environment = arguments + count + 1;

	exit(main(count, arguments, environment));
}
