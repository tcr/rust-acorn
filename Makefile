.PHONY: all

all:
	cargo build

test: all
	node test.js
