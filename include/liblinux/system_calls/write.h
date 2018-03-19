#ifndef LIBLINUX_SYSTEM_CALLS_WRITE_H
#define LIBLINUX_SYSTEM_CALLS_WRITE_H

#include <liblinux/types.h>

ssize_t write(unsigned int file_descriptor, const char * buffer, size_t count);

#endif /* LIBLINUX_SYSTEM_CALLS_WRITE_H */
