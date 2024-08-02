# Compiler
# If CC is not set, use afl-gcc-fast
CC ?= afl-gcc-fast

# Compiler flags
CFLAGS = -Wall -g -fsanitize=address

# Source files
SRC = $(wildcard *.c)

# Object files (change .c to no extension)
OBJ = $(SRC:.c=)

# Default target
all: $(OBJ)

# Rule to compile each .c file into an executable
%: %.c
	$(CC) $(CFLAGS) $< -o $@

# Clean target to remove all executables
clean:
	rm -f $(OBJ)
