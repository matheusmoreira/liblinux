#include <liblinux/system_calls/ioctl.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

#include <linux/termios.h>

#include "common/write/unsigned_integer.c"
#include "common/write/linefeed.c"

#define INPUT 0
#define OUTPUT 1

void write_terminal_size(struct winsize *size)
{
	write_unsigned_integer(OUTPUT, size->ws_col);
	write_linefeed(OUTPUT);
	write_unsigned_integer(OUTPUT, size->ws_row);
	write_linefeed(OUTPUT);
}

int start(int count, char **arguments, char **environment,
		struct auxiliary *values)
{
	struct winsize size = {0};

	ioctl(INPUT, TIOCGWINSZ, &size);
	write_terminal_size(&size);

	return 0;
}
