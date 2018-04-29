#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

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
	write(1, string, length(string));
	write(1, "\n", 1);
}

static void write_c_string_tabbed(const char *string)
{
	write(1, "\t", 1);
	write_c_string(string);
}

static void write_vector(const char *header, char **vector)
{
	write_c_string(header);
	while (*vector)
		write_c_string_tabbed(*vector++);
}

int start(int count, char **arguments, char **environment,
		struct auxiliary *values)
{
	write_vector("Arguments:", arguments);
	write_vector("Environment:", environment);

	return 0;
}
