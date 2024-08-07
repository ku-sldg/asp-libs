asp_name=hsh
source_filename=$(asp_name).c
executable_filename=$(asp_name)_id

all: $(source_filename)
	gcc -w $(source_filename) -o ../bin/$(executable_filename)

clean:
	rm -rf ../bin/$(executable_filename)
