#include <liblinux/system_call.h>
#include <liblinux/system_calls/close.h>
#include <liblinux/definitions.h>

int close(unsigned int file_descriptor)
{
	return system_call(__NR_close, file_descriptor);
}
