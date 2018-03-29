#include <liblinux/system_call.h>
#include <liblinux/system_calls/exit.h>

void exit(int code)
{
	system_call(60, code);
}
