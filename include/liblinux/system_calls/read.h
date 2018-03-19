#ifndef LIBLINUX_SYSTEM_CALLS_READ_H
#define LIBLINUX_SYSTEM_CALLS_READ_H

#include <liblinux/types.h>

long read(unsigned int file_descriptor, char * buffer, size_t count);

#endif /* LIBLINUX_SYSTEM_CALLS_READ_H */
