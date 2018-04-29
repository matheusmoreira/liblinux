#include <liblinux/start.h>

static void *after(void *vector)
{
	void **pointer = (void **) vector;

	while (*pointer++ != 0);

	return pointer;
}

int liblinux_start(void *stack_pointer)
{
	long count;
	char **arguments;
	char **environment;
	struct auxiliary *values;

	count = *((long *) stack_pointer);
	arguments = ((char **) stack_pointer) + 1;
	environment = arguments + count + 1;
	values = after(environment);

	return start(count, arguments, environment, values);
}
