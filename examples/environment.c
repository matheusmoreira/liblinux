#include <liblinux/system_calls/write.h>
#include <liblinux/main.h>

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

int main(int count, char *arguments[], char *environment[])
{
	char **current = arguments;

	write_c_string("Arguments:");
	while (*current)
		write_c_string_tabbed(*current++);

	write_c_string("Environment:");
	current = environment;
	while (*current)
		write_c_string_tabbed(*current++);

	return 0;
}
