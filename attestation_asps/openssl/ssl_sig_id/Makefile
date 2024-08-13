asp_name=ssl_sig
source_filename=$(asp_name).c
executable_filename=$(asp_name)_id
local_bin=../bin/
UNAME := $(shell uname)

all: $(source_filename)
# If we are on Mac, build this way
ifeq ($(UNAME),Darwin)
	ifeq ($(SSL_INCLUDE),) #SSL_INCLUDE env variable undefined
		@echo "Make Error:  missing SSL_INCLUDE environment variable"
		@exit 1
	endif
	ifeq ($(SSL_LIB),)     #SSL_LIB env variable undefined
		@echo "Make Error:  missing SSL_LIB environment variable"
		@exit 1
	endif
	gcc -g -w -I${SSL_INCLUDE} $(source_filename) -o $(local_bin)$(executable_filename) -L${SSL_LIB} -lcrypto -lssl
# Else build normally
else
	gcc -g -w $(source_filename) -o $(local_bin)$(executable_filename) -lcrypto -lssl
endif

clean:
	rm -rf $(local_bin)$(executable_filename)
ifeq ($(UNAME),Darwin)
	rm -rf $(local_bin)$(executable_filename).dSYM
endif
