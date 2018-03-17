# Project file system structure
source_directory := source
include_directory := include
examples_directory := examples

# Directories for build artifacts
build_directory := build
build_objects_directory := $(build_directory)/objects
build_examples_directory := $(build_directory)/$(examples_directory)

# Target is the liblinux shared object
library := liblinux
target := $(build_directory)/$(library).so
architecture := x86_64

# List of C files in source tree
sources := $(source_directory)/arch/$(architecture)/system_call.c \
           $(wildcard $(source_directory)/system_calls/*.c)

# List of object files that will be built
objects := $(sources:$(source_directory)/%.c=$(build_objects_directory)/%.o)

# Library usage examples
examples := $(subst $(examples_directory),\
                    $(build_examples_directory),\
                    $(basename $(wildcard $(examples_directory)/*)))

# Options for GCC
gcc_dialect_options := -ansi -ffreestanding
gcc_warning_options := -Wall -Wextra -Wpedantic
gcc_optimization_options := -Os -fno-strict-aliasing
gcc_preprocessor_options := -I $(include_directory)
gcc_link_options := -nostdlib

gcc_common_options = $(gcc_dialect_options) \
                     $(gcc_warning_options) \
                     $(gcc_optimization_options) \
                     $(gcc_preprocessor_options) \
                     $(gcc_link_options)

gcc_library_directory_option = -L$(1)
gcc_code_generation_options := -fPIC
gcc_compile_option := -c
gcc_shared_library_option := -shared
gcc_output_option = -o $(1)
gcc_link_option = -l $(1)

# Compiler configuration

compiler := gcc
compiler_common_options := $($(compiler)_common_options)

compiler_library_search_options := $(call $(compiler)_library_directory_option,$(build_directory))
compiler_code_generation_options := $($(compiler)_code_generation_options)
compiler_compile_option := $($(compiler)_compile_option)
compiler_shared_library_option := $($(compiler)_shared_library_option)
compiler_output_option = $($(compiler)_output_option)
compiler_link_option = $($(compiler)_link_option)

# Build rules

$(build_objects_directory)/%.o : $(source_directory)/%.c | directories
	$(compiler) \
    $(compiler_common_options) \
    $(compiler_code_generation_options) \
    $(compiler_compile_option) $< \
    $(call compiler_output_option,$@)

$(target) : $(objects) | directories
	$(compiler) \
    $(compiler_common_options) \
    $(compiler_code_generation_options) \
    $(compiler_shared_library_option) \
    $^ \
    $(call compiler_output_option,$@)

$(build_examples_directory)/% : $(examples_directory)/%.c $(target) | directories
	$(compiler) \
    $(compiler_common_options) \
    $< \
    $(compiler_library_search_options) \
    $(call compiler_output_option,$@) \
    $(call compiler_link_option,linux)

# Phony targets

examples: $(examples)

all: $(target) examples

clean:
	rm -r $(build_directory)

directories:
	mkdir -p $(build_objects_directory)/arch/$(architecture) \
             $(build_objects_directory)/system_calls \
             $(build_examples_directory)

.PHONY: examples all clean directories

# Special variables

.DEFAULT_GOAL := all
