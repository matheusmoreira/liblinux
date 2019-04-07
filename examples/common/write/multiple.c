#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_MULTIPLE_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_MULTIPLE_C

#include <liblinux/system_calls/write.h>

static void write_multiple(int fd, void *data, size_t size, int times)
{
	while (times--)
		write(fd, data, size);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_MULTIPLE_C */
