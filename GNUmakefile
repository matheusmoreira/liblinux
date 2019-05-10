library := linux
project := lib$(library)
architecture := x86_64
C.freestanding := yes

headers_library = $(call find,$(include_liblinux_directory),file?)

sources_library = $(call find,$(source_directory),file?)
sources_start = $(call find,$(start_architecture_directory),file?)
sources_examples = $(call glob,$(examples_directory)/*.c)

examples = $(basename $(notdir $(sources_examples)))

objects_static_library = $(call source_to_static_object,$(sources_library))
objects_dynamic_library = $(call source_to_dynamic_object,$(sources_library))
objects_start = $(call source_to_start_object,$(sources_start))
objects_libraries = $(objects_static_library) $(objects_dynamic_library)
objects = $(objects_libraries) $(objects_start)

include make/file
