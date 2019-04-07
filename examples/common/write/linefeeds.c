#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_LINEFEEDS_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_LINEFEEDS_C

#include "multiple.c"

static void write_linefeeds(int fd, int count)
{
	write_multiple(fd, "\n", 1, count);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_LINEFEEDS_C */
