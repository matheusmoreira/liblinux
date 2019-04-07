#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_LINEFEED_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_LINEFEED_C

#include "linefeeds.c"

static void write_linefeed(int fd)
{
	write_linefeeds(fd, 1);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_LINEFEED_C */
