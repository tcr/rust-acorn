.PHONY: all test

test: all
	node test.js

all:
	cargo build
