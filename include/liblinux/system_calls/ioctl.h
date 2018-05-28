#ifndef LIBLINUX_SYSTEM_CALLS_IOCTL_H
#define LIBLINUX_SYSTEM_CALLS_IOCTL_H

#include <linux/ioctl.h>

int ioctl(unsigned int file_descriptor,
		unsigned int command, unsigned long argument);

#endif /* LIBLINUX_SYSTEM_CALLS_IOCTL_H */
