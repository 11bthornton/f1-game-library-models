use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enum representing all possible teams in the game
#[derive(Debug, Copy, Clone, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum Team {
    Mercedes = 0,

    Ferrari = 1,

    RedBullRacing = 2,

    Williams = 3,

    AstonMartin = 4,

    Alpine = 5,

    RB = 6, // Updated from AlphaTauri to RB

    Haas = 7,

    McLaren = 8,

    Sauber = 9, // Updated from AlfaRomeo to Sauber

    Mercedes2020 = 85,

    Ferrari2020 = 86,

    RedBull2020 = 87,

    Williams2020 = 88,

    RacingPoint2020 = 89,

    Renault2020 = 90,

    AlphaTauri2020 = 91,

    Haas2020 = 92,

    McLaren2020 = 93,

    AlfaRomeo2020 = 94,

    AstonMartinDB11V12 = 95,

    AstonMartinVantageF1Edition = 96,

    AstonMartinVantageSafetyCar = 97,

    FerrariF8Tributo = 98,

    FerrariRoma = 99,

    McLaren720S = 100,

    McLarenArtura = 101,

    MercedesAMGGTBlackSeriesSafetyCar = 102,

    MercedesAMGGTRPro = 103,

    F1CustomTeam = 104,

    Prema21 = 106,

    UniVirtuosi21 = 107,

    Carlin21 = 108,

    Hitech21 = 109,

    ArtGP21 = 110,

    MPMotorsport21 = 111,

    Charouz21 = 112,

    Dams21 = 113,

    Campos21 = 114,

    BWT21 = 115,

    Trident21 = 116,

    MercedesAMGGTBlackSeries = 117,

    Mercedes22 = 118,

    Ferrari22 = 119,

    RedBullRacing22 = 120,

    Williams22 = 121,

    AstonMartin22 = 122,

    Alpine22 = 123,

    RB22 = 124, // Updated from AlphaTauri22 to RB22

    Haas22 = 125,

    McLaren22 = 126,

    Sauber22 = 127, // Updated from AlfaRomeo22 to Sauber22

    Konnersport22 = 128,

    Konnersport = 129,

    Prema22 = 130,

    Virtuosi22 = 131,

    Carlin22 = 132,

    MPMotorsport22 = 133,

    Charouz22 = 134,

    Dams22 = 135,

    Campos22 = 136,

    VanAmersfoortRacing22 = 137,

    Trident22 = 138,

    Hitech22 = 139,

    ArtGP22 = 140,

    ArtGP23 = 143,

    Campos23 = 144,

    Carlin23 = 145,

    PHM23 = 146,

    Dams23 = 147,

    Hitech23 = 148,

    MPMotorsport23 = 149,

    Prema23 = 150,

    Trident23 = 151,

    VanAmersfoortRacing23 = 152,

    Virtuosi23 = 153,

    F1Generic = 41, // Added new team

    APXGP24 = 142, // Added new team

    APXGP25 = 154, // Added new team

    Konnersport24 = 155, // Added new team

    ArtGP24 = 158, // Added new team

    Campos24 = 159, // Added new team

    RodinMotorsport24 = 160, // Added new team

    AIXRacing24 = 161, // Added new team

    DAMS24 = 162, // Added new team

    Hitech24 = 163, // Added new team

    MPMotorsport24 = 164, // Added new team

    Prema24 = 165, // Added new team

    Trident24 = 166, // Added new team

    VanAmersfoortRacing24 = 167, // Added new team

    Invicta24 = 168, // Added new team

    Mercedes24 = 185, // Added new team

    Ferrari24 = 186, // Added new team

    RedBullRacing24 = 187, // Added new team

    Williams24 = 188, // Added new team

    AstonMartin24 = 189, // Added new team

    Alpine24 = 190, // Added new team

    RB24 = 191, // Added new team

    Haas24 = 192, // Added new team

    McLaren24 = 193, // Added new team

    Sauber24 = 194, // Added new team

    #[serde(other)]
    None = 255,
}

impl Team {
    pub fn to_string(&self) -> &'static str {
        match self {
            Team::Mercedes => "Mercedes",
            Team::Ferrari => "Ferrari",
            Team::RedBullRacing => "Red Bull Racing",
            Team::Williams => "Williams",
            Team::AstonMartin => "Aston Martin",
            Team::Alpine => "Alpine",
            Team::RB => "RB",
            Team::Haas => "Haas",
            Team::McLaren => "McLaren",
            Team::Sauber => "Sauber",
            Team::Mercedes2020 => "Mercedes 2020",
            Team::Ferrari2020 => "Ferrari 2020",
            Team::RedBull2020 => "Red Bull 2020",
            Team::Williams2020 => "Williams 2020",
            Team::RacingPoint2020 => "Racing Point 2020",
            Team::Renault2020 => "Renault 2020",
            Team::AlphaTauri2020 => "Alpha Tauri 2020",
            Team::Haas2020 => "Haas 2020",
            Team::McLaren2020 => "Mc Laren 2020",
            Team::AlfaRomeo2020 => "Alfa Romeo 2020",
            Team::AstonMartinDB11V12 => "Aston Martin DB11 V12",
            Team::AstonMartinVantageF1Edition => "Aston Martin Vantage F1 Edition",
            Team::AstonMartinVantageSafetyCar => "Aston Martin Vantage Safety Car",
            Team::FerrariF8Tributo => "Ferrari F8 Tributo",
            Team::FerrariRoma => "Ferrari Roma",
            Team::McLaren720S => "Mc Laren 720 S",
            Team::McLarenArtura => "Mc Laren Artura",
            Team::MercedesAMGGTBlackSeriesSafetyCar => "Mercedes AMG GT Black Series Safety Car",
            Team::MercedesAMGGTRPro => "Mercedes AMG GTR Pro",
            Team::F1CustomTeam => "F1 Custom Team",
            Team::Prema21 => "Prema 21",
            Team::UniVirtuosi21 => "Uni Virtuosi 21",
            Team::Carlin21 => "Carlin 21",
            Team::Hitech21 => "Hitech 21",
            Team::ArtGP21 => "Art GP 21",
            Team::MPMotorsport21 => "MPMotorsport 21",
            Team::Charouz21 => "Charouz 21",
            Team::Dams21 => "Dams 21",
            Team::Campos21 => "Campos 21",
            Team::BWT21 => "BWT 21",
            Team::Trident21 => "Trident 21",
            Team::MercedesAMGGTBlackSeries => "Mercedes AMG GT Black Series",
            Team::Mercedes22 => "Mercedes 22",
            Team::Ferrari22 => "Ferrari 22",
            Team::RedBullRacing22 => "Red Bull Racing 22",
            Team::Williams22 => "Williams 22",
            Team::AstonMartin22 => "Aston Martin 22",
            Team::Alpine22 => "Alpine 22",
            Team::RB22 => "RB 22",
            Team::Haas22 => "Haas 22",
            Team::McLaren22 => "McLaren 22",
            Team::Sauber22 => "Sauber 22",
            Team::Konnersport22 => "Konnersport 22",
            Team::Konnersport => "Konnersport",
            Team::Prema22 => "Prema 22",
            Team::Virtuosi22 => "Virtuosi 22",
            Team::Carlin22 => "Carlin 22",
            Team::MPMotorsport22 => "MP Motorsport 22",
            Team::Charouz22 => "Charouz 22",
            Team::Dams22 => "Dams 22",
            Team::Campos22 => "Campos 22",
            Team::VanAmersfoortRacing22 => "Van Amersfoort Racing 22",
            Team::Trident22 => "Trident 22",
            Team::Hitech22 => "Hitech 22",
            Team::ArtGP22 => "Art GP 22",
            Team::ArtGP23 => "Art GP '23",
            Team::Campos23 => "Campos '23",
            Team::Carlin23 => "Carlin '23",
            Team::PHM23 => "PHM '23",
            Team::Dams23 => "Dams '23",
            Team::Hitech23 => "Hitech '23",
            Team::MPMotorsport23 => "MP Motorsport '23",
            Team::Prema23 => "Prema '23",
            Team::Trident23 => "Trident '23",
            Team::VanAmersfoortRacing23 => "Van Amersfoort Racing '23",
            Team::Virtuosi23 => "Virtuosi '23",
            Team::F1Generic => "F1 Generic",
            Team::APXGP24 => "APX GP 24",
            Team::APXGP25 => "APX GP 25",
            Team::Konnersport24 => "Konnersport 24",
            Team::ArtGP24 => "Art GP 24",
            Team::Campos24 => "Campos 24",
            Team::RodinMotorsport24 => "Rodin Motorsport 24",
            Team::AIXRacing24 => "AIX Racing 24",
            Team::DAMS24 => "DAMS 24",
            Team::Hitech24 => "Hitech 24",
            Team::MPMotorsport24 => "MP Motorsport 24",
            Team::Prema24 => "Prema 24",
            Team::Trident24 => "Trident 24",
            Team::VanAmersfoortRacing24 => "Van Amersfoort Racing 24",
            Team::Invicta24 => "Invicta 24",
            Team::Mercedes24 => "Mercedes 24",
            Team::Ferrari24 => "Ferrari 24",
            Team::RedBullRacing24 => "Red Bull Racing 24",
            Team::Williams24 => "Williams 24",
            Team::AstonMartin24 => "Aston Martin 24",
            Team::Alpine24 => "Alpine 24",
            Team::RB24 => "RB 24",
            Team::Haas24 => "Haas 24",
            Team::McLaren24 => "Mc Laren 24",
            Team::Sauber24 => "Sauber 24",
            Team::None => "None",
        }
    }
}
