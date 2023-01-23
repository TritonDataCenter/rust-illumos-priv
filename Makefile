RUST_TOOLCHAIN = 1.49.0

ENGBLD_REQUIRE := $(shell git submodule update --init deps/eng)
include ./deps/eng/tools/mk/Makefile.defs
include ./deps/eng/tools/mk/Makefile.rust.defs

TOP ?= $(error Unable to access eng.git submodule Makefiles.)

.PHONY: test
test: | $(CARGO_EXEC)
	$(CARGO) test

include ./deps/eng/tools/mk/Makefile.targ
include ./deps/eng/tools/mk/Makefile.rust.targ
