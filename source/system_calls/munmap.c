#include <liblinux/system_call.h>
#include <liblinux/system_calls/munmap.h>
#include <liblinux/definitions.h>

int munmap(void *address, size_t length)
{
	return system_call(__NR_munmap, address, length);
}
