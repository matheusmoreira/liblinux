#include <liblinux/start.h>

int liblinux_start(void *stack_pointer)
{
	long count;
	char **arguments;
	char **environment;

	count = *((long *) stack_pointer);
	arguments = ((char **) stack_pointer) + 1;
	environment = arguments + count + 1;

	return start(count, arguments, environment);
}
