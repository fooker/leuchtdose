use clap::{App, SubCommand, ArgMatches};

#[repr(C, packed)]
pub struct Mode {}

impl super::Mode for Mode {
    const ID: u8 = 0;

    fn command() -> App<'static, 'static> {
        SubCommand::with_name("off")
                .about("Turn off")
    }

    fn build(_: &ArgMatches<'static>) -> Self {
        Self {}
    }
}
