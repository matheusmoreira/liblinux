#include <liblinux/system_call.h>
#include <liblinux/system_calls/sysinfo.h>
#include <liblinux/definitions.h>

int sysinfo(struct sysinfo *info)
{
	return system_call(__NR_sysinfo, info);
}
