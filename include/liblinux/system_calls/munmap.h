#ifndef LIBLINUX_SYSTEM_CALLS_MUNMAP_H
#define LIBLINUX_SYSTEM_CALLS_MUNMAP_H

#include <linux/errno.h>

#include <liblinux/types.h>

int munmap(void *address, size_t length);

#endif /* LIBLINUX_SYSTEM_CALLS_MUNMAP_H */
