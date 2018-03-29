#include <liblinux/system_calls/write.h>
#include <liblinux/system_calls/exit.h>

#define OUTPUT 1

void _start(void)
{
	static const char hello_world[] = "Hello, world!" "\n";

	/* No need to write the \0 */
	write(OUTPUT, hello_world, sizeof(hello_world) - 1);

	exit(0);
}
