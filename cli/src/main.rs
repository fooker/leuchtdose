#![feature(conservative_impl_trait)]

#[macro_use]
extern crate clap;

extern crate tint;

extern crate serial;


mod off;
mod solid;
mod breath;
mod rings;
mod emergency;
mod sparkle;

use std::io::Write;
use std::io::Read;

use clap::{App, Arg, ArgMatches, AppSettings};


trait Mode {
    const ID: u8;

    fn command() -> App<'static, 'static>;
    fn build(matches: &ArgMatches<'static>) -> Self;
}


fn write<M>(w: &mut std::io::Write, mode: M) -> std::io::Result<()> where M: Mode + 'static {
    w.write(&[M::ID])?;
    w.write(unsafe {
        std::slice::from_raw_parts((&mode as *const M) as *const u8,
                                   std::mem::size_of::<M>())
    })?;

    return Ok(());
}


fn main() {
    let app = App::new("Leuchtdose-CLI")
            .about("CLI for the leuchtdose")
            .version(crate_version!())
            .bin_name("leuchtdose-cli")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .arg(Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .help("The port where the leuchtdose is attached")
                    .takes_value(true)
                    .required(true))
            .arg(Arg::with_name("timeout")
                    .short("t")
                    .long("time")
                    .help("Disable animation after time")
                    .takes_value(true)
                    .conflicts_with("keypress"))
            .arg(Arg::with_name("keypress")
                    .short("k")
                    .long("key")
                    .help("Disable animation after a key was pressed")
                    .takes_value(false)
                    .conflicts_with("timeout"));

    let app = app.subcommand(off::Mode::command());
    let app = app.subcommand(solid::Mode::command());
    let app = app.subcommand(breath::Mode::command());
    let app = app.subcommand(rings::Mode::command());
    let app = app.subcommand(emergency::Mode::command());
    let app = app.subcommand(sparkle::Mode::command());

    let matches = app.get_matches();

    let mut w: Vec<u8> = vec![];
    match matches.subcommand() {
        ("off", Some(matches)) => write(&mut w, off::Mode::build(matches)),
        ("solid", Some(matches)) => write(&mut w, solid::Mode::build(matches)),
        ("breath", Some(matches)) => write(&mut w, breath::Mode::build(matches)),
        ("rings", Some(matches)) => write(&mut w, rings::Mode::build(matches)),
        ("emergency", Some(matches)) => write(&mut w, emergency::Mode::build(matches)),
        ("sparkle", Some(matches)) => write(&mut w, sparkle::Mode::build(matches)),
        _ => unreachable!(),
    }.expect("Failed to generate data");

    let mut port = serial::open(matches.value_of("port").unwrap()).expect("Failed to open port");
    port.write_all(&w).expect("Failed to send data");
    port.flush().expect("Failed to send data");

    // Handle timeout
    if matches.is_present("timeout") {
        let timeout = matches.value_of("timeout").unwrap()
                .parse::<u64>().expect("timeout must be u64");
        std::thread::sleep(std::time::Duration::from_secs(timeout));

        write(&mut w, off::Mode {}).expect("Failed to generate data");
        port.write_all(&w).expect("Failed to send data");
        port.flush().expect("Failed to send data");
    }

    // Handle keypress
    if matches.is_present("keypress") {
        std::io::stdin().read(&mut [0u8]).unwrap();

        write(&mut w, off::Mode {}).expect("Failed to generate data");
        port.write_all(&w).expect("Failed to send data");
        port.flush().expect("Failed to send data");
    }
}
