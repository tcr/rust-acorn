.PHONY: all

all:
	@cargo build

test: all
	@node test/tests.js
