#include <liblinux/system_call.h>
#include <liblinux/system_calls/close.h>

int close(unsigned int file_descriptor)
{
	return system_call(3, file_descriptor);
}
