/**
 * http://static.rust-lang.org/doc/0.7/extra/getopts.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern mod extra;
use extra::getopts::*;
use std::os;

fn do_work(in: &str, out: Option<~str>) {
	println(in);
	println(match out {
		Some(x) => x,
		None => ~"No Output"
	});
}

fn print_usage(program: &str, _opts: &[Opt]) {
	println(fmt!("Usage: %s [options]", program));
	println("-o\t\tOutput");
	println("-h --help\tUsage");
}

fn main() {
	let args = os::args();

	let program = copy args[0];

	let opts = ~[
		optopt("o"),
		optflag("h"),
		optflag("help")
	];
	let matches = match getopts(args.tail(), opts) {
		Ok(m) => { m }
		Err(f) => {
			fail!(fail_str(f)) }
	};
	if opt_present(&matches, "h") || opt_present(&matches, "help") {
		print_usage(program, opts);
		return;
	}
	let output = opt_maybe_str(&matches, "o");
	let input: &str = if !matches.free.is_empty() {
		copy matches.free[0]
	} else {
		print_usage(program, opts);
		return;
	};
	do_work(input, output);
}
