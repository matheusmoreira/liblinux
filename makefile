# Project file system structure
source_directory := source
include_directory := include
examples_directory := examples

# Directories for build artifacts
build_directory := build
build_objects_directory := $(build_directory)/objects
build_examples_directory := $(build_directory)/$(examples_directory)

# Target is the liblinux shared object
target := liblinux.so
architecture := x86_64

# List of C files in source tree
sources := $(source_directory)/arch/$(architecture)/system_call.c \
           $(wildcard $(source_directory)/system_calls/*.c)

# List of object files that will be built
objects := $(sources:$(source_directory)/%.c=$(build_objects_directory)/%.o)

# Library usage examples
examples := $(wildcard $(examples_directory)/*)

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

# Compiler configuration

compiler := gcc
compiler_options := $($(compiler)_options)

# Variables exported to sub-makes
export build_directory \
       build_examples_directory \
       compiler \
       compiler_options

# Build rules

directories:
	mkdir -p $(build_objects_directory)/arch/$(architecture)
	mkdir -p $(build_objects_directory)/system_calls
	mkdir -p $(build_examples_directory)

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
    $^

# Phony targets

$(examples) : $(build_directory)/$(target)
	$(eval export current_example := $@)
	$(MAKE) --no-print-directory -f $(current_example)/makefile

examples: $(examples)

all: $(build_directory)/$(target) examples

clean:
	rm -r $(build_directory)

.PHONY: $(examples) examples all clean

# Special variables

.DEFAULT_GOAL := all
