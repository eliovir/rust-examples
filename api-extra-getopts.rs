/**
 * http://static.rust-lang.org/doc/master/extra/getopts/index.html
 *
 * @license MIT license <http://www.opensource.org/licenses/mit-license.php>
 */
extern mod extra;
use extra::getopts::{getopts,Opt,optflag,optopt};
use std::os;

fn do_work(inp: &str, out: Option<~str>) {
	println(inp);
	println(match out {
		Some(x) => x,
		None => ~"No Output"
	});
}

fn print_usage(program: &str, _opts: &[Opt]) {
	println!("Usage: {} [options]", program);
	println("-o\\t\\tOutput");
	println("-h --help\\tUsage");
}

fn main() {
	let args = os::args();

	let program = args[0].clone();

	let opts = ~[
		optopt("o"),
		optflag("h"),
		optflag("help")
	];
	let matches = match getopts(args.tail(), opts) {
		Ok(m) => { m }
		Err(f) => { fail!(f.to_err_msg()) }
	};
	if matches.opt_present("h") || matches.opt_present("help") {
		print_usage(program, opts);
		return;
	}
	let output = matches.opt_str("o");
	let input: &str = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		print_usage(program, opts);
		return;
	};
	do_work(input, output);
}
