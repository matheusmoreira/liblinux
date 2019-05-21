library := linux
project := lib$(library)
architecture := x86_64
C.freestanding := yes

# Process start code
objects_start = $(call source_to_start_object,$(sources_start))

# All objects
objects = $(objects_libraries) $(objects_start)

# Library usage examples
sources_examples = $(call glob,$(examples_directory)/*.c)

examples = $(basename $(notdir $(sources_examples)))

# GCC linker specification file and wrapper script
gcc_specs_script := $(scripts_directory)/$(project).specs.sh
gcc_wrapper_script := $(scripts_directory)/$(project)-gcc.sh

include .make/file
