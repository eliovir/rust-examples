rust-examples [![Ohloh statistics](https://www.ohloh.net/p/rust-examples/widgets/project_thin_badge.gif)](https://www.ohloh.net/p/rust-examples)
=============

[Rust-examples](https://github.com/eliovir/rust-examples) is a repository to
gather example codes from tutorial and other documentations of
[Rust](http://www.rust-lang.org/) into files, ready to compile.

Examples are tested with version 0.9-pre.


## Files

* [Homepage](http://www.rust-lang.org/)
    * `what_it_looks_like.rs`
* [Tutorial]
    * 2.1 Compiling your first program: `tutorial-02_1-hello.rs`
    * 3 Syntax basics: `tutorial-03-syntax_basics.rs`
    * 4.3 Loops (`for`, `for i.times`, `while`, `loop`): `tutorial-04_3-loops.rs`
    * 5.2 Enums: `tutorial-05_2-enum.rs`
    * 5.3 Tuples: `tutorial-05_3-tuples.rs`
    * 14 Closures: `tutorial-14-closure.rs`
    * 15 Methods, with *constructor*: `tutorial-15-methods.rs`
    * 17 Generics: `tutorial-17-generics.rs`
* [Rust Tasks and Communication]
    * 2 Basics: `tutorial-tasks-02-basics.rs`
    * 2.1 Communication: `tutorial-tasks-02_1-communication.rs`
    * 2.2 Backgrounding computations: Futures: `tutorial-tasks-02_2-backgrounding_computations.rs`
* [Doc unit testing]
    * Unit testing in Rust: `unittests.rs`
* API
    * Program to an 'interface', not an 'implementation', by [Josh Davis](http://joshldavis.com/2013/07/01/program-to-an-interface-fool/): `lang-interface.rs`
    * Pointer snippets from Dave Herman's talk: `lang-pointers.rs`
    * extra::getopts: `api-extra-getopts.rs`
    * std::hashmap::HashMap: `api-std-hashmap.rs`
    * std::vec: OwnedVector, 2D-arrays, ...: `api-std-vec.rs`
* Some new files:
    * `Makefile` to compile, run tests and run benchmarks
    * A library and its unit tests and benchmarks for 2 Fibonacci functions (a reccursive and a non reccursive): `libfibonacci.rs`
    * A struct to manage dates: `libdate.rs`
    * Design pattern Decorator: `design_pattern-decorator.rs`

[Tutorial]: http://static.rust-lang.org/doc/master/tutorial.html
[Rust Tasks and Communication]: http://static.rust-lang.org/doc/master/tutorial-tasks.html
[Doc unit testing]: https://github.com/mozilla/rust/wiki/Doc-unit-testing

# Compile and running it

You will need the version 0.9-pre of the rust compiler.
If you encounter problems, make sure you have the right version before creating an issue.

The simplest way to build **rust-examples** is to do a clone and use ``make`` to compile:


    git clone https://github.com/eliovir/rust-examples
    cd rust-examples
    make

To run tests and benchmarks:

    make tests
    make bench

To get help on commands:

    make help

## Contributing

1. Fork it (`git clone https://github.com/eliovir/rust-examples`)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Make your changes, and add tests for them
4. Test your changes (`make test`)
5. Commit your changes (`git commit -am 'Add some feature'`)
6. Push to the branch (`git push origin my-new-feature`)
7. Create new Pull Request

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

These codes are distributed under the MIT license.

See LICENSE for details.
