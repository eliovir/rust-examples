rust-examples [![Ohloh statistics](http://www.ohloh.net/p/rust-examples/widgets/project_thin_badge.gif)](https://www.ohloh.net/p/rust-examples)
=============

[![Build Status](http://travis-ci.org/eliovir/rust-examples.png?branch=master)](https://travis-ci.org/eliovir/rust-examples)

[Rust-examples](https://github.com/eliovir/rust-examples) is a repository to
gather example codes from tutorial and other documentations of
[Rust](http://www.rust-lang.org/) into files, ready to compile.

Examples are tested with version 0.11-pre.

## Files

* [Homepage](http://www.rust-lang.org/)
    * `what_it_looks_like.rs`, `what_it_looks_like2.rs`
* [Tutorial]
    * [2.1](http://doc.rust-lang.org/tutorial.html#compiling-your-first-program) Compiling your first program: `tutorial-02_1-hello.rs`
    * [3](http://doc.rust-lang.org/tutorial.html#syntax-basics) Syntax basics: `tutorial-03-syntax_basics.rs`
    * [4.2](http://doc.rust-lang.org/tutorial.html#pattern-matching) Pattern matching: `tutorial-04_2-pattern-matching.rs`
    * [4.3](http://doc.rust-lang.org/tutorial.html#loops) Loops (`for`, `while`, `loop`): `tutorial-04_3-loops.rs`
    * [5.1](http://doc.rust-lang.org/tutorial.html#structs) Structs: `tutorial-05_1-structs.rs`
    * [5.2](http://doc.rust-lang.org/tutorial.html#enums) Enums: `tutorial-05_2-enum.rs`
    * [5.3](http://doc.rust-lang.org/tutorial.html#tuples) Tuples: `tutorial-05_3-tuples.rs`
    * [15](http://doc.rust-lang.org/tutorial.html#closures) Closures: `tutorial-15-closure.rs`
    * [16](http://doc.rust-lang.org/tutorial.html#methods) Methods, with *constructor*: `tutorial-16-methods.rs`
    * [17](http://doc.rust-lang.org/tutorial.html#generics) Generics: `tutorial-17-generics.rs`
* [Rust Tasks and Communication]
    * [2](http://doc.rust-lang.org/guide-tasks.html#basics) Basics: `tutorial-tasks-02-basics.rs`
    * [2.1](http://doc.rust-lang.org/guide-tasks.html#communication) Communication: `tutorial-tasks-02_1-communication.rs`
    * [2.2](http://doc.rust-lang.org/guide-tasks.html#backgrounding-computations:-futures) Backgrounding computations: Futures: `tutorial-tasks-02_2-backgrounding_computations.rs`
    * [2.3](http://doc.rust-lang.org/guide-tasks.html#sharing-immutable-data-without-copy:-arc) Sharing immutable data without copy: Arc: `tutorial-tasks-02_3-arc.rs`
* [Doc unit testing]
    * Unit testing in Rust: `unittests.rs`
* [Rust Cheatsheet]
    * Use Struct to express phantom types: `phantom_type.rs`
* API
    * Program to an 'interface', not an 'implementation', by [Josh Davis](http://joshldavis.com/2013/07/01/program-to-an-interface-fool/): `lang-interface.rs`
    * Lambda expressions: `lang-lambda.rs`
    * Generics, Polymorphism, by [Felix S. Klock II](https://github.com/Rust-Meetup-Paris/Talks/tree/master/introduction_to_rust): `lang-generics.rs`
    * Pointer snippets from Dave Herman's talk: `lang-pointers.rs`
    * [collections:hashmap::HashMap](http://doc.rust-lang.org/collections/hashmap/struct.HashMap.html): `api-collections-hashmap.rs`
    * [getopts](http://doc.rust-lang.org/getopts/index.html): `api-getopts.rs`
    * [std::from_str::FromStr](http://doc.rust-lang.org/std/from_str/trait.FromStr.html): `api-std-from_str.rs`
    * [std::io::File](http://doc.rust-lang.org/std/io/index.html): `api-std-io-file.rs`
    * [std::vec](http://doc.rust-lang.org/std/vec/index.html): OwnedVector, 2D-arrays, ...: `api-std-vec.rs`
* Some new files:
    * `Makefile` to compile, run tests and run benchmarks
    * `.travis.yml` to add the repository to [Travis CI](https://travis-ci.org/eliovir/rust-examples) and [Rust CI](http://www.rust-ci.org/p/90/)
    * A library and its unit tests and benchmarks for 2 Fibonacci functions (a reccursive and a non reccursive): `fibonacci.rs`
    * A struct to manage dates: `date.rs`
    * Different syntaxes to find the maximum value in a vector: `find_max.rs`
    * A struct to manage INI files: `inifile.rs`
    * Design pattern Decorator: `design_pattern-decorator.rs` and `design_pattern-decorator2.rs`
    * Design pattern Strategy: `design_pattern-strategy.rs`
    * Design pattern Observer: `design_pattern-observer.rs`
    * Design pattern Command: `design_pattern-command.rs`
    * Design pattern Template method: `design_pattern-templatemethod.rs`
    * Design pattern Chain of Command: `design_pattern-chain_of_command.rs`

[Tutorial]: http://doc.rust-lang.org/tutorial.html
[The Rust Reference Manual]: http://doc.rust-lang.org/rust.html
[Rust Tasks and Communication]: http://doc.rust-lang.org/guide-tasks.html
[Doc unit testing]: http://doc.rust-lang.org/guide-testing.html
[Rust Cheatsheet]: http://doc.rust-lang.org/complement-cheatsheet.html#how-do-i-express-phantom-types?


# Compile and running it

You will need the version 0.11-pre of the rust compiler.
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

## Links

other projects have similar goals: providing Rust code snippets:

- [Learn Rust](https://github.com/kaseyc/Learn-Rust): Sample programs Kaseyc made to learn the Rust language.
- [Rust design patterns](https://github.com/jdavis/rust-design-patterns): Implementation of various design patterns in Rust. (MIT license)
- [Rust for real](https://github.com/FlaPer87/rust-for-real): Learning Rust by example. (Apache License v2)
- [Rust Rosetta](https://github.com/Hoverbear/rust-rosetta): Implementing [Rosetta Code](http://rosettacode.org/) problems in Rust. (Public domain)
- [Rust sandbox](https://github.com/rntz/rust-sandbox): Playing around with Rust. (DWTFYWT PUBLIC LICENSE v2)
- [Rustlings](https://github.com/smadhueagle/rustlings): Smadheagle's experiments with Rust Programming Language.
- [Rust projects](https://github.com/am0d/rust-projects): Some programs am0d has written as he set about learning rust. (AS-IS)
- [Rust by example](https://github.com/japaric/rust-by-example): Learn Rust with practical examples ([website](http://rustbyexample.com/)). (Apache License v2 and MIT license)

## License

Rust is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

These codes are distributed under the MIT license.

See LICENSE for details.
