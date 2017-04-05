use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;

fn main() {
	// Examine the path of this executable
	let mut args = env::args();
	let program = args.next().unwrap(); 
        let program_name = program.split('/').last().unwrap();

	// Path of the .mark file
	let mut markfile = env::home_dir().unwrap();
	markfile.push(".mark");

	match program_name {
		// mark: Appends its arguments to ~/.mark
		"mark" => {
			let mut file = OpenOptions::new().append(true).create(true).open(&markfile).unwrap();
			for arg in args {
				writeln!(file, "{}", arg).unwrap();
			}
		},
		// unmark: Deletes ~/.mark
		"unmark" => match fs::remove_file(markfile) {
			Ok(_) => {},
			Err(e) => println!("{}", e),
		},
		// marked: Cats ~/.mark
		"marked" => {
			match OpenOptions::new().read(true).open(&markfile) {
				Ok(file) => {
					let file = BufReader::new(file);
					for line in file.lines() {
						println!("{}", line.unwrap());
					}
				},
				Err(e) => if e.kind() != ErrorKind::NotFound {println!("{}", e) },
			}
		},
		irgendwas => println!("Ich weiß leider nicht, was mir \"{}\" sagen soll!", irgendwas),
	}
}
