#include <liblinux/system_calls/write.h>
#include <liblinux/system_calls/exit.h>

void _start(void) {
    static unsigned int standard_output = 1;
    static char hello_world[] = "Hello, world!" "\n";

    write(standard_output, hello_world, sizeof hello_world);
    exit(0);
}
