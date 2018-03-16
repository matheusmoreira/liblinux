#include <liblinux/system_call.h>
#include <liblinux/system_calls/write.h>

long write(unsigned int file_descriptor, char * buffer, size_t count) {
    return system_call(1, file_descriptor, (long) buffer, count);
}
