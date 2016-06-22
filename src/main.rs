extern crate clap;
extern crate term;

use clap::{Arg, App, AppSettings};
use std::ffi::OsStr;
use std::fs::File;
use std::io;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("FILE")
             .multiple(true)
             .required(true)
             .help("The file(s) to display"))
        .get_matches();

    let files: Vec<&OsStr> = matches.values_of_os("FILE").unwrap().collect();
    let mut out = term::stdout().unwrap();
    out.reset().unwrap();

    for (i, &filename) in files.iter().enumerate() {
        if i != 0 {
            println!("");
        }

        if files.len() > 1 {
            out.fg(term::color::GREEN).unwrap();
            out.attr(term::Attr::Bold).unwrap();
            writeln!(out, "{}", filename.to_string_lossy()).unwrap();
            out.reset().unwrap();
        }

        io::copy(&mut File::open(filename).unwrap(), &mut io::stdout()).unwrap();
    }
}
