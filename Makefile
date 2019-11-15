RUST_CODE = 1

ENGBLD_REQUIRE := $(shell git submodule update --init deps/eng)
include ./deps/eng/tools/mk/Makefile.defs
TOP ?= $(error Unable to access eng.git submodule Makefiles.)

test:
	cargo test

include ./deps/eng/tools/mk/Makefile.targ
