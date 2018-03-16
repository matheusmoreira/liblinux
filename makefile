# Project file system structure
build_directory := build
build_objects_directory := $(build_directory)/objects
source_directory := source
include_directory := include

# Target is the liblinux shared object
target := liblinux.so
architecture := x86_64

# List of C files in source tree
sources := $(source_directory)/arch/$(architecture)/system_call.c \
           $(wildcard $(source_directory)/system_calls/*.c)

# List of object files that will be built
objects := $(sources:$(source_directory)/%.c=$(build_objects_directory)/%.o)

# Options for GCC
gcc_standard_options := -ansi
gcc_warning_options := -Wall -Wextra -pedantic
gcc_freestanding_options := -ffreestanding -nostdlib
gcc_include_options := -I $(include_directory)
gcc_library_options := -fPIC
gcc_optimization_options := -fno-strict-aliasing

gcc_options := $(gcc_standard_options) \
               $(gcc_warning_options) \
	           $(gcc_freestanding_options) \
               $(gcc_include_options) \
               $(gcc_library_options) \
               $(gcc_optimization_options)

compiler := gcc
compiler_options := $($(compiler)_options)

directories:
	mkdir -p $(build_objects_directory)/arch/$(architecture)
	mkdir -p $(build_objects_directory)/system_calls

$(build_objects_directory)/%.o : $(source_directory)/%.c | directories
	$(compiler) \
    $(compiler_options) \
    -c \
    -o $@ \
    $<

$(build_directory)/$(target) : $(objects) | directories
	$(compiler) \
    $(compiler_options) \
    -shared \
    -o $@ \
    $<

all: $(build_directory)/$(target)

clean:
	rm -r $(build_directory)

.PHONY: clean
.DEFAULT_GOAL := all