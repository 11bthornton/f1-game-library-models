use std::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Track identifiers for all circuits in F1 games
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(i8)]
pub enum Track {
    Unknown = -1,
    Melbourne = 0,
    PaulRicard = 1,
    Shanghai = 2,
    SakhirBahrain = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hockenheim = 8,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Sochi = 18,
    Mexico = 19,
    BakuAzerbaijan = 20,
    SakhirShort = 21,
    SilverstoneShort = 22,
    TexasShort = 23,
    SuzukaShort = 24,
    Hanoi = 25,
    Zandvoort = 26,
    Imola = 27,
    Portimao = 28,
    Jeddah = 29,
    Miami = 30,
    LasVegas = 31,
    Losail = 32,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Track::Unknown => "Unknown",
            Track::Melbourne => "Melbourne, Australia",
            Track::PaulRicard => "Paul Ricard, France",
            Track::Shanghai => "Shanghai, China",
            Track::SakhirBahrain => "Sakhir, Bahrain",
            Track::Catalunya => "Catalunya, Spain",
            Track::Monaco => "Monaco",
            Track::Montreal => "Montreal, Canada",
            Track::Silverstone => "Silverstone, UK",
            Track::Hockenheim => "Hockenheim, Germany",
            Track::Hungaroring => "Hungaroring, Hungary",
            Track::Spa => "Spa, Belgium",
            Track::Monza => "Monza, Italy",
            Track::Singapore => "Singapore",
            Track::Suzuka => "Suzuka, Japan",
            Track::AbuDhabi => "Abu Dhabi, UAE",
            Track::Texas => "Texas, USA",
            Track::Brazil => "Brazil",
            Track::Austria => "Austria",
            Track::Sochi => "Sochi, Russia",
            Track::Mexico => "Mexico City, Mexico",
            Track::BakuAzerbaijan => "Baku Azerbaijan",
            Track::SakhirShort => "Sakhir Short, Bahrain",
            Track::SilverstoneShort => "Silverstone Short, UK",
            Track::TexasShort => "Texas Short, USA",
            Track::SuzukaShort => "Suzuka Short, Japan",
            Track::Hanoi => "Hanoi, Vietnam",
            Track::Zandvoort => "Zandvoort, Netherlands",
            Track::Imola => "Imola, Italy",
            Track::Portimao => "Portimao, Portugal",
            Track::Jeddah => "Jeddah, Saudi Arabia",
            Track::Miami => "Miami, USA",
            Track::LasVegas => "Las Vegas, USA",
            Track::Losail => "Losail, Qatar",
        };
        write!(f, "{}", name)
    }
}
