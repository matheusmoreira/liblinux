#include <liblinux/system_call.h>
#include <liblinux/system_calls/mmap.h>
#include <liblinux/definitions.h>

void *mmap(void *address, size_t length, int protection, int flags,
           int file_descriptor, off_t offset)
{
	return (void *) system_call(__NR_mmap, address, length,
	                                       protection, flags,
	                                       file_descriptor, offset);
}
