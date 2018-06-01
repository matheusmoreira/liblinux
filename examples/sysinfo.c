#include <liblinux/system_calls/sysinfo.h>
#include <liblinux/system_calls/gettimeofday.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/system_calls/exit.h>
#include <liblinux/start.h>

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

static void write_long(long);
static void write_ulong(unsigned long);

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
	write_long(info->uptime);

	write_message(procs);
	write_ulong(info->procs);

	write_message(load1);
	write_ulong(info->loads[0]);
	write_message(load5);
	write_ulong(info->loads[1]);
	write_message(load15);
	write_ulong(info->loads[2]);

	write_message(mem_unit);
	write_ulong(info->mem_unit);

	write_message(totalram);
	write_ulong(info->totalram * info->mem_unit);
	write_message(freeram);
	write_ulong(info->freeram * info->mem_unit);
	write_message(sharedram);
	write_ulong(info->sharedram * info->mem_unit);
	write_message(bufferram);
	write_ulong(info->bufferram * info->mem_unit);

	write_message(totalswap);
	write_ulong(info->totalswap * info->mem_unit);
	write_message(freeswap);
	write_ulong(info->freeswap * info->mem_unit);

	write_message(totalhigh);
	write_ulong(info->totalhigh * info->mem_unit);
	write_message(freehigh);
	write_ulong(info->freehigh * info->mem_unit);
}

static void write_time(struct timeval *time, struct timezone *zone)
{
	message(seconds, "Seconds since the epoch");
	message(microseconds, "Microseconds since the epoch");

	message(minutes_west, "Minutes west of Greenwich");
	message(dst_type, "Type of DST correction");

	write_message(seconds);
	write_long(time->tv_sec);
	write_message(microseconds);
	write_long(time->tv_usec);

	write_message(minutes_west);
	write_long(zone->tz_minuteswest);
	write_message(dst_type);
	write_long(zone->tz_dsttime);
}

#undef message
#undef write_message

/* Integer to string conversion and output */

static void write_digits(unsigned long n, short negative)
{

/* digits = ceil(bits * log10(2))
 *        = ceil(64   * log10(2))
 *        = 20
 */
#define NUM_DIGITS_64 20

	static char digits[NUM_DIGITS_64 + 2]; /* digits, sign and LF */
	char *digit = digits + NUM_DIGITS_64 + 1; /* work backwards */
	size_t count = 0;

	*digit = '\n'; /* Line ends after writing the number */
	++count;

	do {
		*--digit = '0' + (n % 10);
		n /= 10;
		++count;
	} while (n > 0);

	if (negative) {
		*--digit = '-';
		++count;
	}

	write(OUTPUT, digit, count);
}

static void write_long(long n)
{
	if (n >= 0)
		write_digits(n, 0);
	else
		write_digits(-n, 1);
}

static void write_ulong(unsigned long n)
{
	write_digits(n, 0);
}
