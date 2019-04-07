#include <liblinux/system_calls/sysinfo.h>
#include <liblinux/system_calls/gettimeofday.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/system_calls/exit.h>
#include <liblinux/start.h>

#include "common/write/linefeed.c"
#include "common/write/signed_integer.c"
#include "common/write/unsigned_integer.c"

#define OUTPUT 1
#define ERROR 2

static void handle_errors(int);
static void write_sysinfo(struct sysinfo *);
static void write_time(struct timeval *, struct timezone *);

int start(int count, char **arguments, char **environment,
		struct auxiliary *values)
{
	struct sysinfo info = {0};
	struct timeval time = {0};
	struct timezone zone = {0};

	handle_errors(sysinfo(&info));
	write_sysinfo(&info);

	handle_errors(gettimeofday(&time, &zone));
	write_time(&time, &zone);

	return 0;
}

static void handle_errors(int code)
{
	static const char EFAULT_message[] = "Invalid sysinfo pointer\n";

	switch (code) {
	case -EFAULT:
		write(ERROR, EFAULT_message, sizeof(EFAULT_message) - 1);
		exit(1);
		break;
	}
}

#define message(variable, message) \
	static const char \
	variable##_message[] = message ": "

#define write_message(variable) \
	write(OUTPUT, variable##_message, sizeof(variable##_message) - 1)

static void write_sysinfo(struct sysinfo *info)
{

	message(uptime,    "Uptime (seconds)");

	message(procs,     "Number of running processes");

	message(load1,     "Load average (1 minute)");
	message(load5,     "Load average (5 minutes)");
	message(load15,    "Load average (15 minutes)");

	message(mem_unit,  "Bytes per unit of memory");

	message(totalram,  "Total RAM (bytes)");
	message(freeram,   "Free RAM (bytes)");
	message(sharedram, "Shared RAM (bytes)");
	message(bufferram, "RAM buffers (bytes)");

	message(totalswap, "Total swap (bytes)");
	message(freeswap,  "Free swap (bytes)");

	message(totalhigh, "Total high memory (bytes)");
	message(freehigh,  "Free high memory (bytes)");

	write_message(uptime);
	write_signed_integer(OUTPUT, info->uptime);
	write_linefeed(OUTPUT);

	write_message(procs);
	write_unsigned_integer(OUTPUT, info->procs);
	write_linefeed(OUTPUT);

	write_message(load1);
	write_unsigned_integer(OUTPUT, info->loads[0]);
	write_linefeed(OUTPUT);
	write_message(load5);
	write_unsigned_integer(OUTPUT, info->loads[1]);
	write_linefeed(OUTPUT);
	write_message(load15);
	write_unsigned_integer(OUTPUT, info->loads[2]);
	write_linefeed(OUTPUT);

	write_message(mem_unit);
	write_unsigned_integer(OUTPUT, info->mem_unit);
	write_linefeed(OUTPUT);

	write_message(totalram);
	write_unsigned_integer(OUTPUT, info->totalram * info->mem_unit);
	write_linefeed(OUTPUT);
	write_message(freeram);
	write_unsigned_integer(OUTPUT, info->freeram * info->mem_unit);
	write_linefeed(OUTPUT);
	write_message(sharedram);
	write_unsigned_integer(OUTPUT, info->sharedram * info->mem_unit);
	write_linefeed(OUTPUT);
	write_message(bufferram);
	write_unsigned_integer(OUTPUT, info->bufferram * info->mem_unit);
	write_linefeed(OUTPUT);

	write_message(totalswap);
	write_unsigned_integer(OUTPUT, info->totalswap * info->mem_unit);
	write_linefeed(OUTPUT);
	write_message(freeswap);
	write_unsigned_integer(OUTPUT, info->freeswap * info->mem_unit);
	write_linefeed(OUTPUT);

	write_message(totalhigh);
	write_unsigned_integer(OUTPUT, info->totalhigh * info->mem_unit);
	write_linefeed(OUTPUT);
	write_message(freehigh);
	write_unsigned_integer(OUTPUT, info->freehigh * info->mem_unit);
	write_linefeed(OUTPUT);
}

static void write_time(struct timeval *time, struct timezone *zone)
{
	message(seconds, "Seconds since the epoch");
	message(microseconds, "Microseconds since the epoch");

	message(minutes_west, "Minutes west of Greenwich");
	message(dst_type, "Type of DST correction");

	write_message(seconds);
	write_signed_integer(OUTPUT, time->tv_sec);
	write_linefeed(OUTPUT);
	write_message(microseconds);
	write_signed_integer(OUTPUT, time->tv_usec);
	write_linefeed(OUTPUT);

	write_message(minutes_west);
	write_signed_integer(OUTPUT, zone->tz_minuteswest);
	write_linefeed(OUTPUT);
	write_message(dst_type);
	write_signed_integer(OUTPUT, zone->tz_dsttime);
	write_linefeed(OUTPUT);
}

#undef message
#undef write_message
