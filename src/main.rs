use std::env;
use std::path::Path;
use std::fs::{OpenOptions, read_dir};
use std::io::Write;
use std::ffi::OsStr;

fn fix(file: &Path) {
	if file.is_file() && file.extension().and_then(OsStr::to_str) == Some("mp4") {
		println!("# Fixing: {}", file.display());
		let mut f = OpenOptions::new().write(true).open(file).unwrap();
		f.write_all(b"\x00\x00").unwrap();
	} else if file.is_dir() {
		println!("Scaning directory: {}", file.display());
		for sub in read_dir(file).unwrap() {
			fix(&sub.unwrap().path());
		}
	}
}

fn main() {
	if env::args().len() == 1 {
		fix(Path::new("."));
	} else {
		for argument in env::args().skip(1)
		{
			fix(Path::new(&argument));
		}
	}
}
