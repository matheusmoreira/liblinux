#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_UNSIGNED_INTEGER_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_UNSIGNED_INTEGER_C

#include "integer.c"

static void write_unsigned_integer(int fd, unsigned long n)
{
	write_integer(fd, n, 0);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_UNSIGNED_INTEGER_C */
