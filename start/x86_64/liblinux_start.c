#include <liblinux/start.h>

int liblinux_start(void *stack_pointer)
{
	register long count;
	register char **arguments;
	register char **environment;

	count = *((long *) stack_pointer);
	arguments = ((char **) stack_pointer) + 1;
	environment = arguments + count + 1;

	return start(count, arguments, environment);
}
