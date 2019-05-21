library := linux
project := lib$(library)
architecture := x86_64
C.freestanding := yes

# Library sources and objects
source_directory := source
source_architecture_directory := $(source_directory)/arch/$(architecture)

sources_library = $(call find,$(source_directory),file?)

objects_static_library = $(call source_to_static_object,$(sources_library))
objects_dynamic_library = $(call source_to_dynamic_object,$(sources_library))

objects_libraries = $(objects_static_library) $(objects_dynamic_library)

# Process start code
start_directory := start
start_architecture_directory := $(start_directory)/$(architecture)

sources_start = $(call find,$(start_architecture_directory),file?)

objects_start = $(call source_to_start_object,$(sources_start))

# All objects
objects = $(objects_libraries) $(objects_start)

# Library usage examples
examples_directory := examples

sources_examples = $(call glob,$(examples_directory)/*.c)

examples = $(basename $(notdir $(sources_examples)))

# GCC linker specification file and wrapper script
gcc_specs_script := $(scripts_directory)/$(project).specs.sh
gcc_wrapper_script := $(scripts_directory)/$(project)-gcc.sh

include .make/file
