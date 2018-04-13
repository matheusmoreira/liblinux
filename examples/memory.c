#include <liblinux/system_calls/mmap.h>
#include <liblinux/system_calls/write.h>

#define OUTPUT 1
#define ERROR 2

#define PRINTABLE_COUNT 95
#define PRINTABLE_OFFSET 32

static void *allocate(size_t);

int main(void)
{
	char * text = allocate(PRINTABLE_COUNT + 1);
	int i = 0;

	text[PRINTABLE_COUNT] = '\n';
	for (; i < PRINTABLE_COUNT; ++i) {
		text[i] = PRINTABLE_OFFSET + i;
	}

	if (write(OUTPUT, text, PRINTABLE_COUNT + 1) != (PRINTABLE_COUNT + 1))
		return 1;

	return 0;
}

static void *allocate(size_t size)
{
	return mmap(0, size, PROT_READ | PROT_WRITE,
	            MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
}
