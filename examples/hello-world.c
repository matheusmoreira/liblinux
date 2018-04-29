#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

#define OUTPUT 1

int start(int count, char **arguments, char **environment)
{
	static const char hello_world[] = "Hello, world!\n";

	/* No need to write the \0 */
	write(OUTPUT, hello_world, sizeof(hello_world) - 1);

	return 0;
}
