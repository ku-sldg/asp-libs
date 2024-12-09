SRCDIR := src/bin/
BINDIR := target/debug/
DEST_BINDIR := ../bin/

SOURCES := $(wildcard $(SRCDIR)*.rs)
BASE_NAMES := $(patsubst src/bin/%,%, $(patsubst %.rs,%, $(SOURCES)))

all:
	for asp in $(BASE_NAMES); do \
		cargo build --bin $$asp; \
		cp $(BINDIR)/$$asp $(DEST_BINDIR); \
	done

clean:
	cargo clean
