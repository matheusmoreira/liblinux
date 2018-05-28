#include <liblinux/system_calls/ioctl.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

#include <linux/termios.h>

#define INPUT 0
#define OUTPUT 1

static void write_number(unsigned long number)
{

	/* digits = ceil(bits * log10(2))
	 *        = ceil(64   * log10(2))
	 *        = 20
	 */
	#define NUM_DIGITS_64 20

	static char digits[NUM_DIGITS_64]; /* digits */
	char *digit = digits + NUM_DIGITS_64 - 1; /* work backwards */
	size_t count = 0;

	do {
		*--digit = '0' + (number % 10);
		number /= 10;
		++count;
	} while (number > 0);

	write(OUTPUT, digit, count);
}

void write_terminal_size(struct winsize *size)
{
	write_number(size->ws_col);
	write(OUTPUT, "x", 1);
	write_number(size->ws_row);
	write(OUTPUT, "\n", 1);
}

int start(int count, char **arguments, char **environment,
		struct auxiliary *values)
{
	struct winsize size = {0};

	ioctl(INPUT, TIOCGWINSZ, &size);
	write_terminal_size(&size);

	return 0;
}
