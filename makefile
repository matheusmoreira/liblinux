include make/project
include make/structure

include make/sources

# Target is the liblinux shared object
target := $(build_libraries_directory)/$(project).so
gcc_specs := $(build_libraries_directory)/$(project).specs
gcc_wrapper := $(build_scripts_directory)/$(project)-gcc

# List of object files that will be built
objects_start := $(addsuffix .o,$(basename $(sources_start:$(start_architecture_directory)/%=$(build_start_directory)/%)))
objects_library := $(sources_library:$(source_directory)/%.c=$(build_objects_directory)/%.o)

# Library usage examples
examples := $(basename $(notdir $(wildcard $(examples_directory)/*)))
examples_targets := $(addprefix $(build_examples_directory)/,$(examples))

# Scripts
scripts_linux_directory := $(scripts_directory)/linux

gcc_specs_script := $(scripts_directory)/$(project).specs.sh
gcc_wrapper_script := $(scripts_directory)/$(project)-gcc.sh

download = curl --output $(1) $(2)
download_linux_script = $(call download,$(1),https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/scripts/$(notdir $(1)))

checkpatch.pl := $(scripts_linux_directory)/checkpatch.pl
checkpatch.pl_files := $(addprefix $(dir $(checkpatch.pl)),const_structs.checkpatch spelling.txt)

# Options for GCC
gcc_dialect_options := -ansi -ffreestanding
gcc_warning_options := -Wall -Wextra -Wpedantic
gcc_optimization_options := -Os -fno-strict-aliasing
gcc_instrumentation_options := -fno-stack-protector
gcc_preprocessor_options := -I $(include_directory)

gcc_common_options := $(gcc_dialect_options) \
                      $(gcc_warning_options) \
                      $(gcc_optimization_options) \
                      $(gcc_instrumentation_options) \
                      $(gcc_preprocessor_options)

gcc_library_directory_option = -L$(1)
gcc_code_generation_options := -fPIC
gcc_compile_option := -c
gcc_shared_library_option := -shared
gcc_nostdlib_option := -nostdlib
gcc_output_option = -o $(1)
gcc_link_option = -l $(1)
gcc_specs_option = -specs=$(1)

# Compiler configuration

compiler := gcc
compiler_common_options := $($(compiler)_common_options)

compiler_library_search_options = $($(compiler)_library_directory_option)
compiler_code_generation_options := $($(compiler)_code_generation_options)
compiler_compile_option := $($(compiler)_compile_option)
compiler_shared_library_option := $($(compiler)_shared_library_option)
compiler_nostdlib_option := $($(compiler)_nostdlib_option)
compiler_output_option = $($(compiler)_output_option)
compiler_link_option = $($(compiler)_link_option)

define compiler.compile_object_file
$(compiler) \
$(compiler_common_options) \
$(compiler_nostdlib_option) \
$(compiler_code_generation_options) \
$(call compiler_output_option,$(1)) \
$(compiler_compile_option) $(2)
endef

# Build rules

$(build_objects_directory)/%.o : $(source_directory)/%.c | directories
	$(call compiler.compile_object_file,$@,$<)

define generate_startfile_rule
$(filter %/$(basename $(notdir $(1))).o,$(objects_start)) : $(1) | directories
	$(call compiler.compile_object_file,$$@,$$<)
endef
$(foreach startfile,$(sources_start),$(eval $(call generate_startfile_rule,$(startfile))))
undefine generate_startfile_rule

$(target) : $(objects_library) | directories
	$(compiler) \
    $(compiler_common_options) \
    $(compiler_nostdlib_option) \
    $(compiler_code_generation_options) \
    $(compiler_shared_library_option) \
    $^ \
    $(call compiler_output_option,$@)

$(build_examples_directory)/% : $(examples_directory)/%.c $(target) $(objects_start) $(gcc_wrapper) | directories
	$(gcc_wrapper) \
    $(gcc_common_options) \
    $< \
    $(call gcc_library_directory_option,$(build_libraries_directory)) \
    $(call gcc_output_option,$@) \
    $(call gcc_link_option,$(library))

$(gcc_wrapper) : $(gcc_specs) $(gcc_wrapper_script) | directories
	$(gcc_wrapper_script) $(gcc_specs) > $@
	chmod +x $@

$(gcc_specs) : $(gcc_specs_script) | directories
	$(gcc_specs_script) $(objects_start) > $@

# Script rules

$(checkpatch.pl): $(checkpatch.pl_files)
	$(call download_linux_script,$@)
	chmod +x $@

$(checkpatch.pl_files):
	$(call download_linux_script,$@)

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

define run_example_rule
phony_targets += run-$(1)
run-$(1) : $$(build_examples_directory)/$(1) | directories
	LD_LIBRARY_PATH=$$(build_libraries_directory) $$(build_examples_directory)/$(1)
endef

$(foreach target,$(examples),$(eval $(call run_example_rule,$(target))))

undefine run_example_rule

phony_targets += checkpatch
checkpatch: $(checkpatch.pl) $(checkpatch.pl_files)
	find $(include_directory) $(source_directory) $(examples_directory) \
         -type f \
         -exec $(checkpatch.pl) --quiet --no-tree --file {} \;

# Special variables

.DEFAULT_GOAL := library
.PHONY: $(phony_targets)
