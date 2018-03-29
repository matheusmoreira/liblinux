#include <liblinux/system_call.h>
#include <liblinux/system_calls/sysinfo.h>

int sysinfo(struct sysinfo * info)
{
    return system_call(99, info);
}
