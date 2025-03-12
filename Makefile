BIN := target

# list of targets to not build by default
DEFAULT_EXCLUDES ?= sig_tpm sig_tpm_appr
COMPUTED_EXCLUDES = $(foreach exclude,$(DEFAULT_EXCLUDES),--exclude $(exclude))

default:
	cargo build --release --workspace $(COMPUTED_EXCLUDES)

all: 
	cargo build --release

debug: 
	cargo build 

test:
	make default
	cargo test --workspace $(COMPUTED_EXCLUDES)

clean:
	rm -rf $(BIN)
	cargo clean
