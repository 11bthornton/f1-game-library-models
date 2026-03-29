use std::fmt;

/// Track identifier.
///
/// The wire value is a signed byte; `-1` encodes "unknown".
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[repr(i8)]
pub enum Track {
    Unknown = -1,
    Melbourne = 0,
    Shanghai = 2,
    Sakhir = 3,
    Catalunya = 4,
    Monaco = 5,
    Montreal = 6,
    Silverstone = 7,
    Hungaroring = 9,
    Spa = 10,
    Monza = 11,
    Singapore = 12,
    Suzuka = 13,
    AbuDhabi = 14,
    Texas = 15,
    Brazil = 16,
    Austria = 17,
    Mexico = 19,
    Baku = 20,
    Zandvoort = 26,
    Imola = 27,
    Jeddah = 29,
    Miami = 30,
    LasVegas = 31,
    Losail = 32,
    SilverstoneReverse = 39,
    AustriaReverse = 40,
    ZandvoortReverse = 41,
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Track::Unknown => "Unknown",
            Track::Melbourne => "Melbourne",
            Track::Shanghai => "Shanghai",
            Track::Sakhir => "Sakhir",
            Track::Catalunya => "Catalunya",
            Track::Monaco => "Monaco",
            Track::Montreal => "Montreal",
            Track::Silverstone => "Silverstone",
            Track::Hungaroring => "Hungaroring",
            Track::Spa => "Spa",
            Track::Monza => "Monza",
            Track::Singapore => "Singapore",
            Track::Suzuka => "Suzuka",
            Track::AbuDhabi => "Abu Dhabi",
            Track::Texas => "Texas",
            Track::Brazil => "Brazil",
            Track::Austria => "Austria",
            Track::Mexico => "Mexico",
            Track::Baku => "Baku",
            Track::Zandvoort => "Zandvoort",
            Track::Imola => "Imola",
            Track::Jeddah => "Jeddah",
            Track::Miami => "Miami",
            Track::LasVegas => "Las Vegas",
            Track::Losail => "Losail",
            Track::SilverstoneReverse => "Silverstone (Reverse)",
            Track::AustriaReverse => "Austria (Reverse)",
            Track::ZandvoortReverse => "Zandvoort (Reverse)",
        };
        f.write_str(name)
    }
}
