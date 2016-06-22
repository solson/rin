extern crate clap;

use clap::{Arg, App, AppSettings};
use std::ffi::OsStr;
use std::fs::File;
use std::io;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Scott Olson <scott@solson.me>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("FILE")
             .multiple(true)
             .required(true)
             .help("The file(s) to display"))
        .get_matches();

    let files: Vec<&OsStr> = matches.values_of_os("FILE").unwrap().collect();

    let mut first = true;
    for &filename in &files {
        if first { first = false; } else { println!(""); }

        if files.len() > 1 {
            println!("# {}", filename.to_string_lossy());
        }

        io::copy(&mut File::open(filename).unwrap(), &mut io::stdout()).unwrap();
    }
}
