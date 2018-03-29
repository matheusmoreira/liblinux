#ifndef LIBLINUX_SYSTEM_CALLS_SYSINFO_H
#define LIBLINUX_SYSTEM_CALLS_SYSINFO_H

#include <linux/errno.h>
#include <linux/sysinfo.h>

int sysinfo(struct sysinfo * system_information);

#endif /* LIBLINUX_SYSTEM_CALLS_SYSINFO_H */
