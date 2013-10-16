RUSTC ?= rustc
RUSTFLAGS ?= -O --cfg ndebug

all: build/libfuse.dummy

check: build/fuse_test
	build/fuse_test

clean:
	rm -rf build

.PHONY: all check clean

build:
	mkdir -p $@

build/libfuse.dummy: src/fuse/lib.rs src/fuse/*.rs build
	$(RUSTC) $(RUSTFLAGS) --lib --out-dir $(dir $@) $<

build/fuse_test: src/fuse/lib.rs src/*fuse/*.rs build
	$(RUSTC) $(RUSTFLAGS) --test -o $@ $<


EXAMPLE_SRCS=$(wildcard src/examples/*)
EXAMPLE_BINS=$(patsubst src/examples/%,build/%,$(EXAMPLE_SRCS))

examples: $(EXAMPLE_BINS)

.PHONY: examples

$(EXAMPLE_BINS): build/%: src/examples/%/main.rs build/libfuse.dummy
	$(RUSTC) $(RUSTFLAGS) -L build --bin -o $@ $<
