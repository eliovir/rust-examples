//! https://crates.io/crates/getopts
//! https://docs.rs/getopts/0.2.18/getopts/
//!
//! Tested with rust-1.3.0 and getopts-0.2.14
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-02-09

extern crate getopts;
use getopts::Options;
use std::env;

fn do_work(inp: &str, out: Option<String>) {
	println!("{}", inp);
	match out {
		Some(x) => println!("{}", x),
		None => println!("No Output"),
	}
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} FILE [options]", program);
	print!("{}", opts.usage(&brief));
}

#[cfg(not(test))]
fn main() {
	let args: Vec<String> = env::args().collect();

	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("o", "", "set output file name", "NAME");
	opts.optflag("h", "help", "print this help menu");

	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}
	let output = matches.opt_str("o");
	let input = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		print_usage(&program, opts);
		return;
	};
	do_work(&input, output);
}
