# Project information and file system structure
include make/project
include make/structure

# Integration with project shell scripts
include make/scripts

# List of sources, objects and targets
include make/sources
include make/objects
include make/targets

# Compiler configuration
include make/compiler

# Build rules

$(build_objects_directory)/%.o : $(source_directory)/%.c | directories
	$(call compiler.compile_object,$@,$<)

# Phony targets

phony_targets += library
library: $(target)

phony_targets += startfiles
startfiles: $(objects_start)

phony_targets += examples
examples: $(examples_targets)

phony_targets += all
all: library startfiles examples

phony_targets += clean
clean:
	rm -rf $(build_directory)

phony_targets += directories
directories:
	mkdir -p $(build_architecture_directory) \
             $(build_objects_directory)/system_calls \
             $(build_libraries_directory) \
             $(build_start_directory) \
             $(build_scripts_directory) \
             $(build_examples_directory)

# Special variables

.DEFAULT_GOAL := library
.PHONY: $(phony_targets)
