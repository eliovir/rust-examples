// http://static.rust-lang.org/doc/master/core/ops/index.html
// https://github.com/rust-lang/rust/blob/1.3.0/src/doc/trpl/operators-and-overloading.md
// http://rustbyexample.com/ops.html
// @license MIT license <http://www.opensource.org/licenses/mit-license.php>
use std::ops::Add;
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `Add<T, U>` trait needs two generic parameters:
// * T is the type of the RHS summand, and
// * U is the type of the sum
// This block implements the operation: Foo + Bar = FooBar
impl Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// Addition can be implemented in a non-commutative way
// This block implements the operation: Bar + Foo = BarFoo
impl Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
