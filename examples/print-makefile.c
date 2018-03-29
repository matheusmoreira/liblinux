#include <liblinux/system_calls/open.h>
#include <liblinux/system_calls/close.h>
#include <liblinux/system_calls/read.h>
#include <liblinux/system_calls/write.h>
#include <liblinux/system_calls/exit.h>

#define MAKEFILE "makefile"
#define OUTPUT 1
#define ERROR  2

static char buffer[256];

static void handle_open_errors(int code);
static void handle_read_errors(int code);
static void handle_write_errors(int code);
static void handle_close_errors(int code);

void _start(void)
{
	ssize_t bytes_read = 0, bytes_written = 0;
	int makefile = open(MAKEFILE, O_RDONLY | O_CLOEXEC, 0);

	handle_open_errors(makefile);

	do {
		bytes_read = read(makefile, buffer, sizeof(buffer));
		handle_read_errors(bytes_read);

		bytes_written = write(OUTPUT, buffer, bytes_read);
		handle_write_errors(bytes_written);
	} while (bytes_read > 0);

	handle_close_errors(close(makefile));

	exit(0);
}

/* Error handling macros and functions */

#define error_message(code, message) \
	static const char \
	code##_message[] = function " = -" #code ": " message "\n"

#define error_case(code) \
	case -code:                                                            \
		write(ERROR, code##_message, sizeof(code##_message) - 1);      \
		exit(exit_code);                                               \
		break

static void handle_open_errors(int code)
{

#define function "open"

	error_message(EACCES, "Not permitted to access path");
	error_message(EFAULT, "Path outside accessible address space");
	error_message(EFBIG, "File is too big to be opened");
	error_message(EINTR, "Open interrupted by a signal");
	error_message(EINVAL, "Invalid open flags");
	error_message(EISDIR, "Path refers to a directory");
	error_message(ELOOP, "Too many symbolic links while resolving path");
	error_message(EMFILE, "Per-process open file descriptor limit reached");
	error_message(ENAMETOOLONG, "Path was too long");
	error_message(ENFILE, "System-wide open file descriptor limit reached");
	error_message(ENODEV, "Path refers to device file that doesn't exist");
	error_message(ENOENT, "File referred to by path does not exist");
	error_message(ENOMEM, "No kernel memory available");
	error_message(ENOSPC, "No space left on device");
	error_message(ENOTDIR, "Parts of the path are not directories");
	error_message(ENXIO, "Path refers to device file that doesn't exist");
	error_message(EOVERFLOW, "File is too big to be opened");
	error_message(EPERM, "File seal prevented the file from being opened");

#undef function

#define exit_code 1

	switch (code) {
	error_case(EACCES);
	/* EDQUOT - O_CREAT not specified */
	/* EEXIST - O_CREAT | O_EXCL not specified */
	error_case(EFAULT);
	error_case(EFBIG);
	error_case(EINTR);
	error_case(EINVAL);
	error_case(EISDIR);
	error_case(ELOOP);
	error_case(EMFILE);
	error_case(ENAMETOOLONG);
	error_case(ENFILE);
	error_case(ENODEV);
	error_case(ENOENT);
	error_case(ENOMEM);
	error_case(ENOSPC);
	error_case(ENOTDIR);
	error_case(ENXIO);
	/* EOPNOTSUPP - O_TMPFILE not specified */
	error_case(EOVERFLOW);
	error_case(EPERM);
	/* EROFS - Write access not requested */
	/* ETXTBSY - Write access not requested */
	/* EWOULDBLOCK - O_NONBLOCK not specified */
	}

#undef exit_code

}

static void handle_read_errors(int code)
{

#define function "read"

	error_message(EBADF, "File descriptor not valid or open for reading");
	error_message(EFAULT, "Buffer outside accessible address space");
	error_message(EINTR, "Read interrupted by signal before reading");
	error_message(EINVAL, "File descriptor unsuitable for reading");
	error_message(EIO, "Input error");
	error_message(EISDIR, "File descriptor refers to a directory");

#undef function

#define exit_code 2

	switch (code) {
	/* EAGAIN - O_NONBLOCK not specified */
	/* EWOULDBLOCK - O_NONBLOCK not specified */
	error_case(EBADF);
	error_case(EFAULT);
	error_case(EINTR);
	error_case(EINVAL);
	error_case(EIO);
	error_case(EISDIR);
	}

#undef exit_code

}

static void handle_write_errors(int code)
{

#define function "write"

	error_message(EBADF, "File descriptor not valid or open for writing");
	error_message(EDQUOT, "User's disk quota reached");
	error_message(EFAULT, "Buffer points outside accessible address space");
	error_message(EFBIG, "Maximum file size reached");
	error_message(EINTR, "Write interrupted by signal before writing");
	error_message(EINVAL, "File descriptor unsuitable for writing");
	error_message(EIO, "Output error");
	error_message(ENOSPC, "No space available on device");
	error_message(EPERM, "File seal prevented the file from being written");
	error_message(EPIPE, "The pipe or socket being written to was closed");

#undef function

#define exit_code 3

	switch (code) {
	/* EAGAIN - O_NONBLOCK not specified */
	/* EWOULDBLOCK - O_NONBLOCK not specified */
	error_case(EBADF);
	/* EDESTADDRREQ - File descriptor not a socket */
	error_case(EDQUOT);
	error_case(EFAULT);
	error_case(EFBIG);
	error_case(EINTR);
	error_case(EINVAL);
	error_case(EIO);
	error_case(ENOSPC);
	error_case(EPERM);
	error_case(EPIPE);
	}

#undef exit_code

}

static void handle_close_errors(int code)
{

#define function "close"

	error_message(EBADF, "File descriptor not open or valid");
	error_message(EINTR, "Close interrupted by signal");
	error_message(EIO, "I/O error");
	error_message(ENOSPC, "No space available on device");
	error_message(EDQUOT, "User's disk quota reached");

#undef function

#define exit_code 4

	switch (code) {
	error_case(EBADF);
	error_case(EINTR);
	error_case(EIO);
	error_case(ENOSPC);
	error_case(EDQUOT);
	}

#undef exit_code

}
