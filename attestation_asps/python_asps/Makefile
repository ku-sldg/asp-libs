# Expects that the Python package, pyinstaller, has been installed,
# and is in the search path.
# To install:
#    python3 -m pip install pyinstaller

SRCDIR := ./
BINDIR := dist/
DEST_BINDIR := ../bin/

SOURCE := p_hashfile_id.py
AUX_FILES := copland.py
EXE_NAME := $(patsubst %.py,%, $(SOURCE))

all: $(SOURCE) $(AUX_FILES)
	pyinstaller $(SOURCE) --onefile
	cp $(BINDIR)/$(EXE_NAME) $(DEST_BINDIR)

clean:
	rm -rf $(BINDIR)
	rm -rf build/
