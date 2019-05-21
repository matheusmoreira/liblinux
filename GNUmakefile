library := linux
project := lib$(library)
architecture := x86_64
C.freestanding := yes

# Library usage examples
examples = $(basename $(notdir $(sources_examples)))

# GCC linker specification file and wrapper script
gcc_specs_script := $(scripts_directory)/$(project).specs.sh
gcc_wrapper_script := $(scripts_directory)/$(project)-gcc.sh

include .make/file
