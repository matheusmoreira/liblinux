#include <liblinux/system_call.h>
#include <liblinux/system_calls/write.h>

ssize_t write(unsigned int file_descriptor, const char * buffer, size_t count) {
    return system_call(1, file_descriptor, (long) buffer, count);
}
