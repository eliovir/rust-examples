/**
 * http://static.rust-lang.org/doc/master/getopts/index.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern crate getopts;
use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;

fn do_work(inp: &str, out: Option<~str>) {
	println!("{}", inp);
	match out {
		Some(x) => println!("{}", x),
		None => println!("No Output"),
	}
}

fn print_usage(program: &str, _opts: &[OptGroup]) {
	println!("Usage: {} [options]", program);
	println!("-o\t\tOutput");
	println!("-h --help\tUsage");
}

fn main() {
	let args = os::args();

	let program = args.get(0).clone();

	let opts = ~[
		optopt("o", "", "set output file name", "NAME"),
		optflag("h", "help", "print this help menu")
	];

	let matches = match getopts(args.tail(), opts) {
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};
	if matches.opt_present("h") {
		print_usage(program, opts);
		return;
	}
	let output = matches.opt_str("o");
	let input: &str = if !matches.free.is_empty() {
		(*matches.free.get(0)).clone()
	} else {
		print_usage(program, opts);
		return;
	};
	do_work(input, output);
}
