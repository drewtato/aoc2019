use std::{
	env::args,
	fs,
	io::{stdin, stdout, Write},
	process::Command,
};

const YEAR: u32 = 2019;

const TEMPLATE: &[u8] = br#"
use std::fs::read_to_string;

fn _main() {
	let input: Vec<Vec<isize>> = read_to_string(format!("inputs/day{:02}.txt", DAY))
		.unwrap()
		.trim()
		.lines()
		.map(|l| l.split(',').map(|n| n.trim().parse().unwrap()).collect())
		.collect();
	
	println!("{:?}", input);
}
"#;

type ResultBox<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> ResultBox<()> {
	let args: Vec<String> = args().collect();
	let day: u8 = args[1].parse()?;
	let day_fmt = format!("day{:02}", day);
	let input_file = format!("inputs/{}.txt", day_fmt);
	let code_dir = format!("days/{}", day_fmt);

	if fs::read_dir(&code_dir).is_err() {
		println!("Creating code directory `{}`", code_dir);
		Command::new("cargo").args(&["new", &code_dir]).status()?;

		let mut source = fs::File::create(format!("{}/src/main.rs", code_dir)).unwrap();
		source
			.write_all(format!("const DAY = {};", day).as_bytes())
			.unwrap();
		source.write_all(TEMPLATE).unwrap();
	} else {
		println!("Code directory `{}` already exists", code_dir);
	}
	if fs::File::open(&input_file).is_err() {
		fs::File::create(&input_file)?;
	}

	if fs::read_to_string(&input_file)
		.map(|v| v.trim().is_empty())
		.unwrap_or(true)
	{
		if !bool_input("Fetch input?") {
			return Ok(());
		}

		while let Ok(status) = get_input(day, &input_file) {
			if status < 300 && status >= 200 {
				let cmd = format!("cargo run -p day{:02}", day);

				use clipboard::{ClipboardContext, ClipboardProvider};
				let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
				if ctx.set_contents(cmd.to_owned()).is_ok() {
					println!("Set clipboard!");
				}
				println!("Command to run:\n\n{}\n\nGood luck 🎄", cmd);
				break;
			}

			if bool_input("Try again?") {
				continue;
			} else {
				break;
			}
		}
	} else {
		println!("Input file `{}` already exists", input_file);
	}

	Ok(())
}

fn get_input(day: u8, filename: &str) -> ResultBox<u16> {
	let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
	let cookie = format!(
		"session={}",
		fs::read_to_string("APIKEY").expect("No file called `APIKEY` found")
	);
	let temp = "inputs/tmp.txt";

	let out = Command::new("curl")
		.args(&[
			"-s",
			"-o",
			temp,
			"-w",
			"%{http_code}",
			"--cookie",
			&cookie,
			&url,
		])
		.output()?;

	let status: u16 = String::from_utf8_lossy(&out.stdout).trim().parse()?;

	if !(status < 300 && status >= 200) {
		eprintln!("Status {}; Message:\n{}", status, fs::read_to_string(temp)?);
		fs::remove_file(temp)?;
	} else {
		fs::rename(temp, filename)?;
	}

	Ok(status)
}

fn bool_input(message: &str) -> bool {
	loop {
		print!("{} (y/n) > ", message);
		stdout().flush().unwrap();
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		match input.to_ascii_lowercase().trim() {
			"y" => return true,
			"n" => return false,
			_ => continue,
		}
	}
}
