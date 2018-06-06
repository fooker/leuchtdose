use clap::{App, Arg, SubCommand, ArgMatches};
use tint::Color;

#[repr(C, packed)]
pub struct Mode {
    r: u8,
    g: u8,
    b: u8,
}

impl super::Mode for Mode {
    const ID: u8 = 1;

    fn command() -> App<'static, 'static> {
        SubCommand::with_name("solid")
                .about("Solid color")
                .arg(Arg::with_name("color")
                        .short("c")
                        .long("color")
                        .help("The color to set")
                        .takes_value(true)
                        .default_value("purple"))
    }

    fn build(matches: &ArgMatches<'static>) -> Self {
        let color = Color::from(matches.value_of("color").unwrap()).to_rgb255();

        Self {
            r: color.0,
            g: color.1,
            b: color.2,
        }
    }
}
