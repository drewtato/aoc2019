use std::{
	env::args,
	fs::{read_to_string, remove_file, File},
	process::Command,
};

const YEAR: u32 = 2019;

type ResultBox<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> ResultBox<()> {
	let args: Vec<String> = args().collect();
	unimplemented!();
	Ok(())
}

fn get_input(day: u8, filename: &str) -> ResultBox<u16> {
	let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
	let cookie = format!(
		"session={}",
		read_to_string("APIKEY").expect("No file called `APIKEY` found")
	);
	let out = Command::new("curl")
		.args(&[
			"-s",
			"-o",
			filename,
			"-w",
			"%{http_code}",
			"--cookie",
			&cookie,
			&url,
		])
		.output()?;
	let status: u16 = String::from_utf8_lossy(&out.stdout).trim().parse()?;
	if !(status < 300 && status >= 200) {
		eprintln!("Status {}; Message:\n{}", status, read_to_string(filename)?);
		remove_file(filename)?;
	}
	Ok(status)
}
