#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_SIGNED_INTEGER_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_SIGNED_INTEGER_C

#include "integer.c"

static void write_signed_integer(int fd, long n)
{
	write_integer(fd, n, n < 0);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_SIGNED_INTEGER_C */
