#ifndef LIBLINUX_SYSTEM_CALLS_MMAP_H
#define LIBLINUX_SYSTEM_CALLS_MMAP_H

#include <linux/mman.h>
#include <linux/errno.h>

#include <liblinux/types.h>

void *mmap(void *address, size_t length, int protection, int flags,
           int file_descriptor, off_t offset);

#endif /* LIBLINUX_SYSTEM_CALLS_MMAP_H */
