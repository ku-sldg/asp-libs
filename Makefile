# List of directories

# Find all directories that have a Makefile in them
DIRS := $(shell find . -mindepth 2 -maxdepth 2 -type f -name Makefile | xargs -n 1 dirname)

IGNORED_DIRS := testing

BIN := ./bin

# Target to run make in each directory
.PHONY: all $(DIRS) 

all: $(DIRS)

$(DIRS):
	if ! echo $(IGNORED_DIRS) | grep -qw $@; then \
		mkdir -p $(BIN); \
		$(MAKE) -C $@; \
		cp $@/bin/* $(BIN); \
	fi

# mkdir -p $(BIN)
# for dir in $(DIRS); do \
# 	$(MAKE) -C $$dir; \
# 	cp $$dir/bin/* $(BIN); \
# done

# Optional: Define a clean target to clean all directories
.PHONY: clean

clean:
	if [ $(IGNORED_DIRS) != $@ ]; then \
		$(MAKE) -C $@ clean; \
	fi
	rm -rf $(BIN); 
# for dir in $(DIRS); do \
# 	$(MAKE) -C $$dir clean; \
# done
# rm -rf $(BIN)
