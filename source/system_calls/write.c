#include <liblinux/system_calls/write.h>

extern long system_call_6(long, long, long, long, long, long, long);

long write(unsigned int file_descriptor, char * buffer, size_t count) {
    return system_call_6(1, file_descriptor, (long) buffer, count, 0, 0, 0);
}