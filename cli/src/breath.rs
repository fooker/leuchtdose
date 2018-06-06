use clap::{App, Arg, SubCommand, ArgMatches};
use tint::Color;


#[repr(C, packed)]
pub struct Mode {
    r: u8,
    g: u8,
    b: u8,
    speed: u8,
}

impl super::Mode for Mode {
    const ID: u8 = 2;

    fn command() -> App<'static, 'static> {
        SubCommand::with_name("breath")
                .about("Fade color in and out")
                .arg(Arg::with_name("color")
                        .short("c")
                        .long("color")
                        .help("The color to set")
                        .takes_value(true)
                        .default_value("green"))
                .arg(Arg::with_name("speed")
                        .short("s")
                        .long("speed")
                        .help("Speed")
                        .default_value("5")
                        .takes_value(true))
    }

    fn build(matches: &ArgMatches<'static>) -> Self {
        let color = Color::from(matches.value_of("color").unwrap()).to_rgb255();

        let speed = matches.value_of("speed").unwrap()
                           .parse::<u8>().expect("speed must be u8");

        Self {
            r: color.0,
            g: color.1,
            b: color.2,
            speed,
        }
    }
}
