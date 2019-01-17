# Project information and file system structure
include make/project
include make/structure

# Custom make functions for convenience
include make/functions

# Integration with project shell scripts
include make/scripts

# Compiler configuration
include make/compiler

# List of sources, objects and targets
include make/sources
include make/objects
include make/targets

# Phony targets
include make/phony

# Special variables

.DEFAULT_GOAL := libraries
.PHONY: $(phony_targets)
