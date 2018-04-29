#ifndef LIBLINUX_START_H
#define LIBLINUX_START_H

#include <liblinux/auxiliary.h>

int start(int count, char **arguments, char **environment,
		struct auxiliary *values);

#endif /* LIBLINUX_START_H */
