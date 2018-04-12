#include <liblinux/system_call.h>
#include <liblinux/system_calls/read.h>
#include <liblinux/definitions.h>

ssize_t read(unsigned int file_descriptor, char *buffer, size_t count)
{
	return system_call(__NR_read, file_descriptor, (long) buffer, count);
}
