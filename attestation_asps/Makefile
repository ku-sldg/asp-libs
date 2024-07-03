# List of directories
DIRS := test_asp attest_id certificate_id appraise_id

BIN := ./bin

# Target to run make in each directory
.PHONY: all $(DIRS)

all: $(DIRS)

$(DIRS):
	mkdir -p $(BIN)
	$(MAKE) -C $@

# Optional: Define a clean target to clean all directories
.PHONY: clean

clean:
	for dir in $(DIRS); do \
		$(MAKE) -C $$dir clean; \
	done
	rm -rf $(BIN)
