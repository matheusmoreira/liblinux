#include <liblinux/system_calls/mmap.h>
#include <liblinux/system_calls/munmap.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/start.h>

#define PRINTABLE_COUNT 95
#define PRINTABLE_OFFSET 32

struct memory {
	size_t length;
	void *address;
};

static void allocate(struct memory *, size_t);
static void deallocate(struct memory *);
static ssize_t output(struct memory *);

int start(int count, char **arguments, char **environment,
		struct auxiliary *values)
{
	struct memory data = {0};
	char *text = 0;
	int i = 0;

	allocate(&data, PRINTABLE_COUNT + 1);

	text = data.address;
	text[PRINTABLE_COUNT] = '\n';
	for (; i < PRINTABLE_COUNT; ++i)
		text[i] = PRINTABLE_OFFSET + i;

	if (output(&data) != data.length)
		return 1;

	deallocate(&data);

	return 0;
}

static void allocate(struct memory *data, size_t length)
{
	data->length = length;
	data->address = mmap(0, length, PROT_READ | PROT_WRITE,
	                     MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
}

static void deallocate(struct memory *data)
{
	munmap(data->address, data->length);
	data->address = 0;
	data->length = 0;
}

#define OUTPUT 1

static ssize_t output(struct memory *data)
{
	return write(OUTPUT, data->address, data->length);
}
