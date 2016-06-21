extern crate clap;

use clap::{Arg, App};
use std::ffi::OsStr;
use std::fs::File;
use std::io;

fn main() {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Scott Olson <scott@solson.me>")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("FILE")
             .multiple(true)
             .required(true)
             .help("The file(s) to display"));

    if std::env::args().len() == 1 {
        app.print_help().unwrap();
        println!("");
        return;
    }

    let matches = app.get_matches();
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
