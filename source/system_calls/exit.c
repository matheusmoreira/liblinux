#include <liblinux/system_call.h>
#include <liblinux/system_calls/exit.h>
#include <liblinux/definitions.h>

void exit(int code)
{
	system_call(__NR_exit, code);
}
