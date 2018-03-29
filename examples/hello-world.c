#include <liblinux/system_calls/write.h>
#include <liblinux/system_calls/exit.h>

#define OUTPUT 1

void _start(void)
{
	static const char hello_world[] = "Hello, world!" "\n";

	write(OUTPUT, hello_world, sizeof(hello_world));

	exit(0);
}
