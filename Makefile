sources = $(wildcard *.rs)

ifeq ($(OS),Windows_NT)
	execs = $(patsubst %.rs,%.exe,$(sources))
	EXE_EXT := .exe
else
	execs = $(patsubst %.rs,%,$(sources))
	EXE_EXT = 
endif

CC = rustc

all: $(execs)

%$(EXE_EXT): %.rs
	$(CC) $< -o $@

clean:
ifeq ($(OS),Windows_NT)
	del /S /Q *.exe *.pdb
else
	rm -rf $(execs)
endif