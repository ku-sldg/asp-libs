asp_name=ssl_sig_appr
source_filename=$(asp_name).c
executable_filename=$(asp_name)_id
local_bin=../bin/

all: $(source_filename)
    ifeq ($(SSL_INCLUDE),) #SSL_INCLUDE env variable undefined
		@echo "Make Error:  missing SSL_INCLUDE environment variable"
		@exit 1
    endif
    ifeq ($(SSL_LIB),)     #SSL_LIB env variable undefined
		@echo "Make Error:  missing SSL_LIB environment variable"
		@exit 1
    endif
	gcc -g -w -I${SSL_INCLUDE} $(source_filename) -o $(local_bin)$(executable_filename) -L${SSL_LIB} -lcrypto -lssl

clean:
	rm -rf $(local_bin)$(executable_filename)
	rm -rf $(local_bin)$(executable_filename).dSYM
