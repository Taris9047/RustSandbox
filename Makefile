sources = $(wildcard *.rs)
execs = $(patsubst %.rs,%,$(sources))

CC = rustc

all: $(execs)

%: %.rs
	$(CC) $< -o $@


clean:
	rm -rf $(execs)
