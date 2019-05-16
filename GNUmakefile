library := linux
project := lib$(library)
architecture := x86_64
C.freestanding := yes

# Library headers
include_directory := include
include_liblinux_directory := $(include_directory)/liblinux

headers_library = $(call find,$(include_liblinux_directory),file?)

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
scripts_directory := scripts

gcc_specs_script := $(scripts_directory)/$(project).specs.sh
gcc_wrapper_script := $(scripts_directory)/$(project)-gcc.sh

# Linux kernel checkpatch script integration
scripts_linux_directory := $(scripts_directory)/linux

checkpatch.pl := $(scripts_linux_directory)/checkpatch.pl
checkpatch_data_files := $(addprefix $(dir $(checkpatch.pl)),const_structs.checkpatch spelling.txt)

linux_script_url = https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/scripts/$(1)
download_linux_script = $(call download,$(call linux_script_url,$(notdir $(1))),$(1))

$(checkpatch.pl): $(checkpatch_data_files)
	$(call download_linux_script,$@)
	chmod +x $@

$(checkpatch_data_files):
	$(call download_linux_script,$@)

phony_targets += checkpatch
checkpatch: $(checkpatch.pl)
	find $(include_directory) $(source_directory) $(start_directory) $(examples_directory) \
	     -type f \
	     -exec $(checkpatch.pl) --quiet --no-tree --file {} \;

include make/file
