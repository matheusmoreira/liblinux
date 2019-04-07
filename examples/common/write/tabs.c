#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_TABS_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_TABS_C

#include "multiple.c"

static void write_tabs(int fd, int count)
{
	write_multiple(fd, "\t", 1, count);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_TABS_C */
