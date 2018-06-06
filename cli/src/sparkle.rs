use clap::{App, Arg, SubCommand, ArgMatches};
use tint::Color;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entry {
    r: u8,
    g: u8,
    b: u8,
}

#[repr(C, packed)]
pub struct Mode {
    fade: u8,
    chance: u8,
    palette: [Entry; 16],
}

impl super::Mode for Mode {
    const ID: u8 = 5;

    fn command() -> App<'static, 'static> {
        SubCommand::with_name("sparkle")
                .about("Sparkling random colors")
                .arg(Arg::with_name("fade")
                        .short("f")
                        .long("fade")
                        .help("Fading speed")
                        .default_value("10")
                        .takes_value(true))
                .arg(Arg::with_name("chance")
                        .short("x")
                        .long("chance")
                        .help("Chance of new values")
                        .default_value("10")
                        .takes_value(true))
                .arg(Arg::with_name("color")
                        .short("c")
                        .long("color")
                        .help("Use a single color palette")
                        .takes_value(true)
                        .conflicts_with_all(&["palette", "preset"]))
                .arg(Arg::with_name("palette")
                        .long("palette")
                        .help("Use a custom palette")
                        .takes_value(true)
                        .number_of_values(16)
                        .conflicts_with_all(&["color", "preset"]))
                .arg(Arg::with_name("preset")
                        .short("p")
                        .long("preset")
                        .help("Use a palette preset")
                        .takes_value(true)
                        .possible_values(&["rainbow", "rainbow_stripes", "party", "heat", "cloud", "lava", "ocean", "forest"])
                        .conflicts_with_all(&["color", "palette"]))
    }

    fn build(matches: &ArgMatches<'static>) -> Self {
        let fade = matches.value_of("fade").unwrap()
                .parse::<u8>().expect("fade must be u8");
        let chance = matches.value_of("chance").unwrap()
                          .parse::<u8>().expect("chance must be u8");

        if let Some(color) = matches.value_of("color") {
            let (r, g, b) = Color::from(color).to_rgb255();
            return Mode {
                fade,
                chance,
                palette: [Entry { r, g, b }; 16],
            };
        } else if let Some(colors) = matches.values_of("palette") {
            // TODO: Can we map/collect this?
            let mut palette = [Entry { r: 0, g: 0, b: 0 }; 16];
            for (i, color) in colors.enumerate() {
                let (r, g, b) = Color::from(color).to_rgb255();
                palette[i] = Entry { r, g, b };
            }
            return Mode { fade, chance, palette };
        } else if let Some(preset) = matches.value_of("preset") {
            return Mode {
                fade,
                chance,
                palette: match preset {
                    "rainbow" => RAINBOW_PALETTE,
                    "party" => PARTY_PALETTE,
                    "heat" => HEAT_PALETTE,
                    "cloud" => CLOUD_PALETTE,
                    "lava" => LAVA_PALETTE,
                    "ocean" => OCEAN_PALETTE,
                    "forest" => FOREST_PALETTE,
                    _ => unreachable!(),
                }
            };
        } else {
            unreachable!()
        };
    }
}

// Palettes are shameless stolen from FastLED

const AQUA: Entry = Entry { r: 0x00, g: 0xFF, b: 0xFF };
const AQUAMARINE: Entry = Entry { r: 0x7F, g: 0xFF, b: 0xD4 };
const BLACK: Entry = Entry { r: 0x00, g: 0x00, b: 0x00 };
const BLUE: Entry = Entry { r: 0x00, g: 0x00, b: 0xFF };
const CADET_BLUE: Entry = Entry { r: 0x5F, g: 0x9E, b: 0xA0 };
const CORNFLOWER_BLUE: Entry = Entry { r: 0x64, g: 0x95, b: 0xED };
const DARK_BLUE: Entry = Entry { r: 0x00, g: 0x00, b: 0x8B };
const DARK_CYAN: Entry = Entry { r: 0x00, g: 0x8B, b: 0x8B };
const DARK_GREEN: Entry = Entry { r: 0x00, g: 0x64, b: 0x00 };
const DARK_OLIVE_GREEN: Entry = Entry { r: 0x55, g: 0x6B, b: 0x2F };
const DARK_RED: Entry = Entry { r: 0x8B, g: 0x00, b: 0x00 };
const FOREST_GREEN: Entry = Entry { r: 0x22, g: 0x8B, b: 0x22 };
const GREEN: Entry = Entry { r: 0x00, g: 0x80, b: 0x00 };
const LAWN_GREEN: Entry = Entry { r: 0x7C, g: 0xFC, b: 0x00 };
const LIGHT_BLUE: Entry = Entry { r: 0xAD, g: 0xD8, b: 0xE6 };
const LIGHT_GREEN: Entry = Entry { r: 0x90, g: 0xEE, b: 0x90 };
const LIGHT_SKY_BLUE: Entry = Entry { r: 0x87, g: 0xCE, b: 0xFA };
const LIME_GREEN: Entry = Entry { r: 0x32, g: 0xCD, b: 0x32 };
const MAROON: Entry = Entry { r: 0x80, g: 0x00, b: 0x00 };
const MEDIUM_AQUAMARINE: Entry = Entry { r: 0x66, g: 0xCD, b: 0xAA };
const MEDIUM_BLUE: Entry = Entry { r: 0x00, g: 0x00, b: 0xCD };
const MIDNIGHT_BLUE: Entry = Entry { r: 0x19, g: 0x19, b: 0x70 };
const NAVY: Entry = Entry { r: 0x00, g: 0x00, b: 0x80 };
const OLIVE_DARB: Entry = Entry { r: 0x6B, g: 0x8E, b: 0x23 };
const ORANGE: Entry = Entry { r: 0xFF, g: 0xA5, b: 0x00 };
const RED: Entry = Entry { r: 0xFF, g: 0x00, b: 0x00 };
const SEA_GREEN: Entry = Entry { r: 0x2E, g: 0x8B, b: 0x57 };
const SKY_BLUE: Entry = Entry { r: 0x87, g: 0xCE, b: 0xEB };
const TEAL: Entry = Entry { r: 0x00, g: 0x80, b: 0x80 };
const WHITE: Entry = Entry { r: 0xFF, g: 0xFF, b: 0xFF };
const YELLOW_GREEN: Entry = Entry { r: 0x9A, g: 0xCD, b: 0x32 };

const CLOUD_PALETTE: [Entry; 16] = [
    BLUE,
    DARK_BLUE,
    DARK_BLUE,
    DARK_BLUE,
    DARK_BLUE,
    DARK_BLUE,
    DARK_BLUE,
    DARK_BLUE,
    BLUE,
    DARK_BLUE,
    SKY_BLUE,
    SKY_BLUE,
    LIGHT_BLUE,
    WHITE,
    LIGHT_BLUE,
    SKY_BLUE,
];

const LAVA_PALETTE: [Entry; 16] = [
    BLACK,
    MAROON,
    BLACK,
    MAROON,
    DARK_RED,
    MAROON,
    DARK_RED,
    DARK_RED,
    DARK_RED,
    RED,
    ORANGE,
    WHITE,
    ORANGE,
    RED,
    DARK_RED,
    BLACK,
];


const OCEAN_PALETTE: [Entry; 16] = [
    MIDNIGHT_BLUE,
    DARK_BLUE,
    MIDNIGHT_BLUE,
    NAVY,
    DARK_BLUE,
    MEDIUM_BLUE,
    SEA_GREEN,
    TEAL,
    CADET_BLUE,
    BLUE,
    DARK_CYAN,
    CORNFLOWER_BLUE,
    AQUAMARINE,
    SEA_GREEN,
    AQUA,
    LIGHT_SKY_BLUE,
];

const FOREST_PALETTE: [Entry; 16] = [
    DARK_GREEN,
    DARK_GREEN,
    DARK_OLIVE_GREEN,
    DARK_GREEN,
    GREEN,
    FOREST_GREEN,
    OLIVE_DARB,
    GREEN,
    SEA_GREEN,
    MEDIUM_AQUAMARINE,
    LIME_GREEN,
    YELLOW_GREEN,
    LIGHT_GREEN,
    LAWN_GREEN,
    MEDIUM_AQUAMARINE,
    FOREST_GREEN,
];

const RAINBOW_PALETTE: [Entry; 16] = [
    Entry { r: 0xFF, g: 0x00, b: 0x00 },
    Entry { r: 0xD5, g: 0x2A, b: 0x00 },
    Entry { r: 0xAB, g: 0x55, b: 0x00 },
    Entry { r: 0xAB, g: 0x7F, b: 0x00 },
    Entry { r: 0xAB, g: 0xAB, b: 0x00 },
    Entry { r: 0x56, g: 0xD5, b: 0x00 },
    Entry { r: 0x00, g: 0xFF, b: 0x00 },
    Entry { r: 0x00, g: 0xD5, b: 0x2A },
    Entry { r: 0x00, g: 0xAB, b: 0x55 },
    Entry { r: 0x00, g: 0x56, b: 0xAA },
    Entry { r: 0x00, g: 0x00, b: 0xFF },
    Entry { r: 0x2A, g: 0x00, b: 0xD5 },
    Entry { r: 0x55, g: 0x00, b: 0xAB },
    Entry { r: 0x7F, g: 0x00, b: 0x81 },
    Entry { r: 0xAB, g: 0x00, b: 0x55 },
    Entry { r: 0xD5, g: 0x00, b: 0x2B }
];

const PARTY_PALETTE: [Entry; 16] = [
    Entry { r: 0x55, g: 0x00, b: 0xAB },
    Entry { r: 0x84, g: 0x00, b: 0x7C },
    Entry { r: 0xB5, g: 0x00, b: 0x4B },
    Entry { r: 0xE5, g: 0x00, b: 0x1B },
    Entry { r: 0xE8, g: 0x17, b: 0x00 },
    Entry { r: 0xB8, g: 0x47, b: 0x00 },
    Entry { r: 0xAB, g: 0x77, b: 0x00 },
    Entry { r: 0xAB, g: 0xAB, b: 0x00 },
    Entry { r: 0xAB, g: 0x55, b: 0x00 },
    Entry { r: 0xDD, g: 0x22, b: 0x00 },
    Entry { r: 0xF2, g: 0x00, b: 0x0E },
    Entry { r: 0xC2, g: 0x00, b: 0x3E },
    Entry { r: 0x8F, g: 0x00, b: 0x71 },
    Entry { r: 0x5F, g: 0x00, b: 0xA1 },
    Entry { r: 0x2F, g: 0x00, b: 0xD0 },
    Entry { r: 0x00, g: 0x07, b: 0xF9 }
];

const HEAT_PALETTE: [Entry; 16] = [
    Entry { r: 0x00, g: 0x00, b: 0x00 },
    Entry { r: 0x33, g: 0x00, b: 0x00 },
    Entry { r: 0x66, g: 0x00, b: 0x00 },
    Entry { r: 0x99, g: 0x00, b: 0x00 },
    Entry { r: 0xCC, g: 0x00, b: 0x00 },
    Entry { r: 0xFF, g: 0x00, b: 0x00 },
    Entry { r: 0xFF, g: 0x33, b: 0x00 },
    Entry { r: 0xFF, g: 0x66, b: 0x00 },
    Entry { r: 0xFF, g: 0x99, b: 0x00 },
    Entry { r: 0xFF, g: 0xCC, b: 0x00 },
    Entry { r: 0xFF, g: 0xFF, b: 0x00 },
    Entry { r: 0xFF, g: 0xFF, b: 0x33 },
    Entry { r: 0xFF, g: 0xFF, b: 0x66 },
    Entry { r: 0xFF, g: 0xFF, b: 0x99 },
    Entry { r: 0xFF, g: 0xFF, b: 0xCC },
    Entry { r: 0xFF, g: 0xFF, b: 0xFF }
];
