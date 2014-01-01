RUSTC=echo -e "\033[32;1mRustc:\033[33m" $@ "\033[m"; rustc
LIBSRC=date.rs fibonacci.rs inifile.rs
LIBSTAMP=$(patsubst %.rs,lib-stamps/%,$(LIBSRC))
TESTSRC=$(LIBSRC) unittests.rs
TESTPROG:=$(patsubst %.rs,build/test-%,$(TESTSRC))
SRC=$(wildcard *.rs)
SRC:=$(filter-out $(TESTSRC),$(SRC))
PROG:=$(patsubst %.rs,build/%,$(SRC))
RUSTFLAGS=

.SILENT:
.PRECIOUS: $(LIBSTAMP)

all: exe test bench
	# Build executables, run tests and benchmarks

help:
	# Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

clean:
	# Remove executables, test files, libraries
	rm -fr $(PROG) $(TESTPROG) *~ build/ html-doc/ lib/ lib-stamps/

bench: $(TESTPROG)
	# Run benchmarks
	@for EXE in $(TESTPROG); do\
		./$$EXE --bench;\
	done

doc: $(SRC)
	# Run rustdoc
	@for FI in $(SRC); do \
		rustdoc -o html-doc $$FI;\
	done

exe: $(PROG)
	# Build executables

test: $(TESTPROG)
	# Run tests
	@for EXE in $(TESTPROG); do\
		./$$EXE;\
	done

build:
	mkdir -p $@

build/tutorial-tasks-02_2-backgrounding_computations: tutorial-tasks-02_2-backgrounding_computations.rs $(LIBSTAMP) build
	$(RUSTC) $(RUSTFLAGS) $< -o $@ -L lib/

build/% : %.rs build
	$(RUSTC) $(RUSTFLAGS) $< -o $@

lib:
	mkdir -p $@

lib-stamps/% : %.rs lib
	mkdir -p lib-stamps ;
	$(RUSTC) --out-dir lib/ --lib $<  > $@;

build/test-% : %.rs $(LIBSTAMP) build
	$(RUSTC) $(RUSTFLAGS) $< -o $@ --test
