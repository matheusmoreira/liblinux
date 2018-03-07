#include <liblinux/system_calls/exit.h>

extern long system_call_6(long, long, long, long, long, long, long);

void exit(int code) {
    system_call_6(60, code, 0, 0, 0, 0, 0);
}