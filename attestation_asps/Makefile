# List of directories
DIRS := test_asp

# Target to run make in each directory
.PHONY: all $(DIRS)

all: $(DIRS)

$(DIRS):
	$(MAKE) -C $@

# Optional: Define a clean target to clean all directories
.PHONY: clean

clean:
	for dir in $(DIRS); do \
		$(MAKE) -C $$dir clean; \
	done
