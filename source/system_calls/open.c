#include <liblinux/system_call.h>
#include <liblinux/system_calls/open.h>
#include <liblinux/definitions.h>

int open(const char *path, int flags, mode_t mode)
{
	return system_call(__NR_open, path, flags, mode);
}
