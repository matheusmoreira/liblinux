#ifndef LIBLINUX_EXAMPLES_COMMON_WRITE_INTEGER_C
#define LIBLINUX_EXAMPLES_COMMON_WRITE_INTEGER_C

#include <liblinux/system_calls/write.h>

#include "../decimal_digits.h"

static void write_integer(int fd, unsigned long n, short is_negative)
{
	static char digits[DECIMAL_DIGITS_64_BITS + 1]; /* digits, sign */
	char *digit = digits + DECIMAL_DIGITS_64_BITS;  /* work backwards */
	size_t count = 0;

	do {
		*--digit = '0' + (n % 10);
		n /= 10;
		++count;
	} while (n > 0);

	if (is_negative) {
		*--digit = '-';
		++count;
	}

	write(fd, digit, count);
}

#endif /* LIBLINUX_EXAMPLES_COMMON_WRITE_INTEGER_C */
