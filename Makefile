RUSTC=printf "\033[32;1mRustc:\033[33m %s\033[m\n" $@; rustc
LIBSRC:=$(shell grep -l 'crate_type = "lib"' *rs)
LIBSTAMP=$(patsubst %.rs,lib-stamps/%,$(LIBSRC))
TESTSRC:=$(shell grep -l '\#\[test\]' *.rs)
TESTPROG:=$(patsubst %.rs,build/test-%,$(TESTSRC))
SRC=$(wildcard *.rs)
SRC:=$(filter-out unittests.rs,$(SRC))
PROG:=$(patsubst %.rs,build/%,$(SRC))
RUSTFLAGS=

.SILENT:
.PRECIOUS: $(LIBSTAMP)

all: exe test bench trailing
	# Build executables, run tests and benchmarks

help:
	# Show this help
	grep -A1 ^[a-z].*\: Makefile | sed -r 's/: (.*)$$/:/g' | sed ':a;N;$$!ba;s/:\n//g' | sed s,\\#,\\t,g | grep -v \\--

clean:
	# Remove executables, test files, libraries
	rm -fr $(PROG) $(TESTPROG) *~ build/ doc/ lib/ lib-stamps/

bench: $(TESTPROG)
	# Run benchmarks
	@for EXE in $(TESTPROG); do\
		./$$EXE --bench;\
	done

docs: $(SRC) $(LIBSRC)
	# Run rustdoc
	@for FI in $(SRC) $(LIBSRC); do \
		rustdoc -o doc $$FI;\
	done

exe: $(PROG)
	# Build executables

test: $(TESTPROG)
	# Run tests
	@EXIT=0; for EXE in $(TESTPROG); do\
		./$$EXE; RET=$$?; \
		if test "$$RET" != "0"; then \
			EXIT=$$RET;\
		fi;\
	done; exit $$EXIT

trailing: $(SRC)
	# Check trailing spaces
	@NB=0; for FI in $(SRC); do \
		grep -n '\s\+$$' $$FI; RET=$$?; \
		if test "$$RET" == "0"; then \
			echo $$FI; \
			NB=1; \
		fi; \
	done; exit $$NB

version:
	# Display version of source code
	git describe

build/tutorial-tasks-02_2-backgrounding_computations: tutorial-tasks-02_2-backgrounding_computations.rs $(LIBSTAMP) build
	$(RUSTC) $(RUSTFLAGS) $< -o $@ -L lib/

build/% : %.rs
	mkdir -p build
	$(RUSTC) $(RUSTFLAGS) $< -o $@


lib-stamps/% : %.rs
	mkdir -p lib
	mkdir -p lib-stamps ;
	$(RUSTC) --out-dir lib/ $<  > $@;

build/test-% : %.rs $(LIBSTAMP)
	mkdir -p build
	$(RUSTC) $(RUSTFLAGS) $< -o $@ --test -L lib/
