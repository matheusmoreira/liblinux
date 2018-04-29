#ifndef LIBLINUX_AUXILIARY_H
#define LIBLINUX_AUXILIARY_H

#include <linux/auxvec.h>
#include <linux/elf.h>

#if defined(__x86_64__)
typedef Elf64_Off elf_addr_t;
#else
typedef Elf32_Off elf_addr_t;
#endif

struct auxiliary {
	elf_addr_t type;
	elf_addr_t value;
};

#endif /* LIBLINUX_AUXILIARY_H */
