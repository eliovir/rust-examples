rust-examples
=============

[Rust-examples](https://github.com/eliovir/rust-examples) is a repository to
gather example codes from tutorial and other documentations of
[Rust](http://www.rust-lang.org/) into files, ready to compile.

Examples are tested with version 0.7.

## Files

* [Homepage](http://www.rust-lang.org/)
  * `what_it_looks_like.rs`
* [tutorial]
  * 2.1 Compiling your first program: `tutorial-02_1-hello.rs`
  * 3 Syntax basics: `tutorial-03-syntax_basics.rs`
  * 5.2 Enums: `tutorial-05_2-enum.rs`
* [tutorial-tasks](Rust Tasks and Communication)
  * 2 Basics: `tutorial-tasks-02-basics.rs`
  * 2.2 Backgrounding computations: Futures: `tutorial-tasks-02_2-backgrounding_computations.rs`
* [unittesting](Doc unit testing)
  * Unit testing in Rust: `unittests.rs`
* Some new files:
  * `Makefile` to compile, run tests and run benchmarks
  * A library and tests for a Fibonacci function: `libfibonacci.rs` 

[tutorial]: http://static.rust-lang.org/doc/0.7/tutorial.html
[tutorial-tasks]: http://static.rust-lang.org/doc/0.7/tutorial-tasks.html
[unittesting]: https://github.com/mozilla/rust/wiki/Doc-unit-testing

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

These codes are distributed under the MIT license.

See LICENSE for details.

# Running it

To compile executables, simply

```bash
$ make exe
```

To get help on commands:

```bash
$ make help
```

