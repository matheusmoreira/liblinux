#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

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

static void write_number(unsigned long n)
{

	/* digits = ceil(bits * log10(2))
	 *        = ceil(64   * log10(2))
	 *        = 20
	 */
	#define NUM_DIGITS_64 20

	static char digits[NUM_DIGITS_64 + 1]; /* digits and LF */
	char *digit = digits + NUM_DIGITS_64; /* work backwards */
	size_t count = 0;

	*digit = '\n'; /* Line ends after writing the number */
	++count;

	do {
		*--digit = '0' + (n % 10);
		n /= 10;
		++count;
	} while (n > 0);

	write(1, "\t\t", 2);
	write(1, digit, count);
}

static void write_random(unsigned char *bytes, size_t count)
{
	size_t i;

	for (i = 0; i < count; ++i)
		write_number(bytes[i]);
}

static void write_auxiliary(struct auxiliary *values)
{
	write_c_string("Auxiliary vector:");

	for (;; ++values) {
		switch (values->type) {

		#define common_case(type, header)                              \
		case type:                                                     \
			write_c_string_tabbed(header)                          \

		#define generic_case(type)                                     \
		common_case(type, #type);

		#define number_case(type)                                      \
		common_case(type, #type);                                      \
			write_number(values->value);                           \
			break

		#define string_case(type)                                      \
		common_case(type, #type);                                      \
			write(1, "\t", 1);                                     \
			write_c_string_tabbed((char *) values->value);         \
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
			write_random((unsigned char *) values->value, 16);
			break;

		#undef common_case
		#undef generic_case
		#undef number_case
		#undef string_case

		default:
			write_c_string_tabbed("Unknown auxiliary value");
			write_number(values->type);
			write_number(values->value);
			break;

		}

		if (values->type == AT_NULL)
			break;
	}
}
