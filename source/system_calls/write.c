#include <liblinux/system_call.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/definitions.h>

ssize_t write(unsigned int file_descriptor, const char *buffer, size_t count)
{
	return system_call(__NR_write, file_descriptor, (long) buffer, count);
}
