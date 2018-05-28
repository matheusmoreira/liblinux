#include <liblinux/system_call.h>
#include <liblinux/system_calls/ioctl.h>
#include <liblinux/definitions.h>

int ioctl(unsigned int file_descriptor,
		unsigned int command, unsigned long argument)
{
        return system_call(__NR_ioctl, file_descriptor, command, argument);
}
