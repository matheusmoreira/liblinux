#ifndef LIBLINUX_TYPES_H
#define LIBLINUX_TYPES_H

#include <linux/posix_types.h>

typedef __kernel_size_t size_t;
typedef __kernel_ssize_t ssize_t;

typedef __kernel_mode_t mode_t;

typedef __kernel_off_t off_t;

#endif /* LIBLINUX_TYPES_H */
