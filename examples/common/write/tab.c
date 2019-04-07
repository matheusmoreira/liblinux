#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_TAB_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_TAB_C

#include "tabs.c"

static void write_tab(int fd)
{
	write_tabs(fd, 1);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_TAB_C */
