#include <liblinux/system_call.h>
#include <liblinux/system_calls/open.h>

int open(const char *path, int flags, mode_t mode)
{
	return system_call(2, path, flags, mode);
}
