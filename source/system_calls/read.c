#include <liblinux/system_call.h>
#include <liblinux/system_calls/read.h>

long read(unsigned int file_descriptor, char * buffer, size_t count) {
    return system_call(0, file_descriptor, (long) buffer, count);
}
