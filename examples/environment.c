#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

#include "common/write/linefeed.c"
#include "common/write/tabs.c"
#include "common/write/tab.c"
#include "common/write/unsigned_integer.c"

#define OUTPUT 1

static void write_vector(const char *, char **);
static void write_auxiliary(struct auxiliary *);

int start(int count, char **arguments, char **environment,
		struct auxiliary *values)
{
	write_vector("Arguments:", arguments);
	write_vector("Environment:", environment);

	write_auxiliary(values);

	return 0;
}

static size_t length(const char *beginning)
{
	const char *end = beginning;

	if (!end)
		return 0;

	while (*end)
		++end;

	return (end - beginning);
}

static void write_c_string(const char *string)
{
	write(OUTPUT, string, length(string));
}

static void write_c_string_tabbed(const char *string)
{
	write_tab(OUTPUT);
	write_c_string(string);
}

static void write_vector(const char *header, char **vector)
{
	write_c_string(header);
	write_linefeed(OUTPUT);

	while (*vector) {
		write_c_string_tabbed(*vector++);
		write_linefeed(OUTPUT);
	}
}

static void write_random(unsigned char *bytes, size_t count)
{
	size_t i;

	for (i = 0; i < count; ++i) {
		write_unsigned_integer(OUTPUT, bytes[i]);
		write(OUTPUT, " ", 1);
	}
}

static void write_auxiliary(struct auxiliary *values)
{
	write_c_string("Auxiliary vector:");
	write_linefeed(OUTPUT);

	for (;; ++values) {
		switch (values->type) {

		#define common_case(type, header)                              \
		case type:                                                     \
			write_c_string_tabbed(header);                         \
			write_linefeed(OUTPUT)                                 \

		#define generic_case(type)                                     \
		common_case(type, #type);

		#define number_case(type)                                      \
		common_case(type, #type);                                      \
			write_tabs(OUTPUT, 2);                                 \
			write_unsigned_integer(OUTPUT, values->value);         \
			write_linefeed(OUTPUT);                                \
			break

		#define string_case(type)                                      \
		common_case(type, #type);                                      \
			write_tab(OUTPUT);                                     \
			write_c_string_tabbed((char *) values->value);         \
			write_linefeed(OUTPUT);                                \
			break

		number_case(AT_NULL);
		number_case(AT_IGNORE);
		number_case(AT_EXECFD);
		number_case(AT_PHDR);
		number_case(AT_PHENT);
		number_case(AT_PHNUM);
		number_case(AT_PAGESZ);
		number_case(AT_BASE);
		number_case(AT_FLAGS);
		number_case(AT_ENTRY);
		number_case(AT_NOTELF);
		number_case(AT_UID);
		number_case(AT_EUID);
		number_case(AT_GID);
		number_case(AT_EGID);
		string_case(AT_PLATFORM);
		number_case(AT_HWCAP);
		number_case(AT_CLKTCK);
		number_case(AT_SECURE);
		string_case(AT_BASE_PLATFORM);
		number_case(AT_HWCAP2);
		string_case(AT_EXECFN);

		generic_case(AT_RANDOM)
			write_tabs(OUTPUT, 2);
			write_random((unsigned char *) values->value, 16);
			write_linefeed(OUTPUT);
			break;

		#undef common_case
		#undef generic_case
		#undef number_case
		#undef string_case

		default:
			write_c_string_tabbed("Unknown auxiliary value");

			write_tabs(OUTPUT, 2);
			write_unsigned_integer(OUTPUT, values->type);
			write_linefeed(OUTPUT);

			write_tabs(OUTPUT, 2);
			write_unsigned_integer(OUTPUT, values->value);
			write_linefeed(OUTPUT);
			break;

		}

		if (values->type == AT_NULL)
			break;
	}
}
