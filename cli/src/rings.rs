use clap::{App, Arg, SubCommand, ArgMatches};
use tint::Color;

#[repr(u8)]
pub enum Direction {
    DOWN = 0,
    UP = 1,
}

#[repr(C, packed)]
pub struct Mode {
    r: u8,
    g: u8,
    b: u8,
    direction: Direction,
    speed: u8,
}

impl super::Mode for Mode {
    const ID: u8 = 3;

    fn command() -> App<'static, 'static> {
        SubCommand::with_name("rings")
                .about("Rings moving up or down the bulb")
                .arg(Arg::with_name("color")
                        .short("c")
                        .long("color")
                        .help("The color to set")
                        .takes_value(true)
                        .default_value("white"))
                .arg(Arg::with_name("direction")
                        .short("d")
                        .long("direction")
                        .help("Going up or down")
                        .takes_value(true)
                        .possible_values(&["up", "down"])
                        .default_value("down"))
                .arg(Arg::with_name("speed")
                        .short("s")
                        .long("speed")
                        .help("Speed")
                        .default_value("16")
                        .takes_value(true))
    }

    fn build(matches: &ArgMatches<'static>) -> Self {
        let color = Color::from(matches.value_of("color").unwrap()).to_rgb255();

        let direction = match matches.value_of("direction").unwrap() {
            "up" => Direction::UP,
            "down" => Direction::DOWN,
            _ => unreachable!(),
        };

        let speed = matches.value_of("speed").unwrap()
                          .parse::<u8>().expect("speed must be u8");

        Self {
            r: color.0,
            g: color.1,
            b: color.2,
            direction,
            speed,
        }
    }
}
