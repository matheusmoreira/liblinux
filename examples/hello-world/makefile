$(build_examples_directory)/hello-world : $(current_example)/hello-world.c
	$(compiler) \
    $(compiler_common_options) \
    $< \
    $(compiler_output_option) $@ \
    $(compiler_library_search_options) \
    $(compiler_link_option) linux
