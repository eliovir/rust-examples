#![feature(test)]
#[cfg(test)]
extern crate test;

#[cfg(test)]
use test::Bencher;

#[bench]
fn bench_inifile(b: &mut Bencher) {
	b.iter(|| {
		let mut ini = IniFile::new();
		ini.read("data/config.ini");
	});
}
