$(build_examples_directory)/hello-world : $(current_example)/hello-world.c
	$(compiler) \
    $(compiler_options) \
    -L$(build_directory) \
    -o $@ \
    $< \
    -l linux
