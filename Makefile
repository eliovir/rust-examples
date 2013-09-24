RUSTC=echo -e "\033[32;1mRustc:\033[33m" $@ "\033[m"; rustc
SRC=$(wildcard *.rs)
PROG=$(patsubst %.rs,%,$(SRC))
CXXFLAGS=

.SILENT:

all: $(PROG)

clean:
	rm -fr $(PROG) *~ html-doc

% : %.rs
	$(RUSTC) $(CXXFLAGS) $< -o $@ 
