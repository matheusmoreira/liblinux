#ifndef LIBLINUX_SYSTEM_CALLS_GETTIMEOFDAY_H
#define LIBLINUX_SYSTEM_CALLS_GETTIMEOFDAY_H

#include <linux/time.h>

int gettimeofday(struct timeval *time_value, struct timezone *time_zone);

#endif /* LIBLINUX_SYSTEM_CALLS_GETTIMEOFDAY_H */
