/// Track identifier.
///
/// The wire value is a signed byte; `-1` encodes "unknown".
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::TryFromPrimitive)]
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
