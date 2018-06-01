#include <liblinux/system_call.h>
#include <liblinux/system_calls/gettimeofday.h>
#include <liblinux/definitions.h>

long gettimeofday(struct timeval *tv, struct timezone *tz)
{
	return system_call(__NR_gettimeofday, tv, tz);
}
