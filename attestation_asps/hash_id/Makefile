all: hash.c
    ifeq ($(SSL_INCLUDE),)
		@echo "Make Error:  missing SSL_INCLUDE environment variable"
		@exit 1
    endif
    ifeq ($(SSL_LIB),)
		@echo "Make Error:  missing SSL_LIB environment variable"
		@exit 1
    endif
	gcc -g -I${SSL_INCLUDE}  hash.c -o ../bin/hash_id -L${SSL_LIB} -lcrypto -lssl
clean:
	rm ../bin/hash_id
