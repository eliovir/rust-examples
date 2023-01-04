CARGO=printf "\033[32;1mCargo:\033[33m %s\033[m\n" $@; cargo
BENCH=printf "\033[32;1mBench:\033[33m %s\033[m\n" $@; cargo bench --features=nightly --bin
RUSTC=printf "\033[32;1mRustc:\033[33m %s\033[m\n" $@; rustc
LIBSRC:=$(shell grep -l 'crate_type = "lib"' *rs)
LIBSTAMP=$(patsubst %.rs,lib-stamps/%,$(LIBSRC))
TESTSRC:=$(shell grep -l '\#\[test\]' *.rs)
TESTPROG:=$(patsubst %.rs,build/test-%,$(TESTSRC))
BENCHMARKS=fibonacci find_max
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
	rm -fr $(PROG) $(TESTPROG) *.o *~ build/ doc/ lib/ lib-stamps/ target/

bench:
	# Run benchmarks
	@for EXE in $(BENCHMARKS); do\
		$(BENCH) $$EXE;\
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
		if [ "$$RET" -ne "0" ]; then \
			EXIT=$$RET;\
		fi;\
	done; exit $$EXIT

trailing: $(SRC)
	# Check trailing spaces
	@NB=0; for FI in $(SRC); do \
		grep -n '\s\+$$' $$FI; RET=$$?; \
		if [ "$$RET" -eq "0" ]; then \
			echo $$FI; \
			NB=1; \
		fi; \
	done; exit $$NB

version:
	# Display version of source code
	git describe

build/api-getopts: api-getopts.rs build
	$(CARGO) build --bin api-getopts
	cp target/debug/api-getopts $@

build/api-rand: api-rand.rs build
	$(CARGO) build --bin api-rand
	cp target/debug/api-rand $@

build/book-3_1-guessing_game: book-3_1-guessing_game.rs build
	$(CARGO) build --bin book-3_1-guessing_game
	cp target/debug/book-3_1-guessing_game $@

build/inifile: inifile.rs build
	$(CARGO) build --bin inifile
	cp target/debug/inifile $@

build/tutorial-tasks-02_2-backgrounding_computations: tutorial-tasks-02_2-backgrounding_computations.rs $(LIBSTAMP) build
	$(RUSTC) $(RUSTFLAGS) $< -o $@ -L lib/

build/tutorial-tasks-02_3-arc: tutorial-tasks-02_3-arc.rs build
	$(CARGO) build --bin tutorial-tasks-02_3-arc
	cp target/debug/tutorial-tasks-02_3-arc $@

build/tutorial-04_2-pattern-matching: tutorial-04_2-pattern-matching.rs build
	$(CARGO) build --bin tutorial-04_2-pattern-matching
	cp target/debug/tutorial-04_2-pattern-matching $@

build:
	mkdir -p build

build/% : %.rs
	mkdir -p build
	$(RUSTC) $(RUSTFLAGS) $< -o $@


lib-stamps/% : %.rs
	mkdir -p lib
	mkdir -p lib-stamps ;
	$(RUSTC) --out-dir lib/ $<  > $@;

lib-stamps/inifilex : inifile.rs
	mkdir -p lib
	mkdir -p lib-stamps ;
	$(CARGO) build --lib inifile

build/test-% : %.rs $(LIBSTAMP)
	mkdir -p build
	$(RUSTC) $(RUSTFLAGS) $< -o $@ --test -L lib/
