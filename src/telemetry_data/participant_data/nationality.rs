use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enum representing all possible nationalities in the game
#[derive(Deserialize_repr, Debug, Serialize_repr, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Nationality {
    /// Default nationality
    Default = 0,

    /// American nationality
    American = 1,

    /// Argentinean nationality
    Argentinean = 2,

    /// Australian nationality
    Australian = 3,

    /// Austrian nationality
    Austrian = 4,

    /// Azerbaijani nationality
    Azerbaijani = 5,

    /// Bahraini nationality
    Bahraini = 6,

    /// Belgian nationality
    Belgian = 7,

    /// Bolivian nationality
    Bolivian = 8,

    /// Brazilian nationality
    Brazilian = 9,

    /// British nationality
    British = 10,

    /// Bulgarian nationality
    Bulgarian = 11,

    /// Cameroonian nationality
    Cameroonian = 12,

    /// Canadian nationality
    Canadian = 13,

    /// Chilean nationality
    Chilean = 14,

    /// Chinese nationality
    Chinese = 15,

    /// Colombian nationality
    Colombian = 16,

    /// Costa Rican nationality
    CostaRican = 17,

    /// Croatian nationality
    Croatian = 18,

    /// Cypriot nationality
    Cypriot = 19,

    /// Czech nationality
    Czech = 20,

    /// Danish nationality
    Danish = 21,

    /// Dutch nationality
    Dutch = 22,

    /// Ecuadorian nationality
    Ecuadorian = 23,

    /// English nationality
    English = 24,

    /// Emirian nationality
    Emirian = 25,

    /// Estonian nationality
    Estonian = 26,

    /// Finnish nationality
    Finnish = 27,

    /// French nationality
    French = 28,

    /// German nationality
    German = 29,

    /// Ghanaian nationality
    Ghanaian = 30,

    /// Greek nationality
    Greek = 31,

    /// Guatemalan nationality
    Guatemalan = 32,

    /// Honduran nationality
    Honduran = 33,

    /// Hong Konger nationality
    HongKonger = 34,

    /// Hungarian nationality
    Hungarian = 35,

    /// Icelander nationality
    Icelander = 36,

    /// Indian nationality
    Indian = 37,

    /// Indonesian nationality
    Indonesian = 38,

    /// Irish nationality
    Irish = 39,

    /// Israeli nationality
    Israeli = 40,

    /// Italian nationality
    Italian = 41,

    /// Jamaican nationality
    Jamaican = 42,

    /// Japanese nationality
    Japanese = 43,

    /// Jordanian nationality
    Jordanian = 44,

    /// Kuwaiti nationality
    Kuwaiti = 45,

    /// Latvian nationality
    Latvian = 46,

    /// Lebanese nationality
    Lebanese = 47,

    /// Lithuanian nationality
    Lithuanian = 48,

    /// Luxembourger nationality
    Luxembourger = 49,

    /// Malaysian nationality
    Malaysian = 50,

    /// Maltese nationality
    Maltese = 51,

    /// Mexican nationality
    Mexican = 52,

    /// Monegasque nationality
    Monegasque = 53,

    /// New Zealander nationality
    NewZealander = 54,

    /// Nicaraguan nationality
    Nicaraguan = 55,

    /// Northern Irish nationality
    NorthernIrish = 56,

    /// Norwegian nationality
    Norwegian = 57,

    /// Omani nationality
    Omani = 58,

    /// Pakistani nationality
    Pakistani = 59,

    /// Panamanian nationality
    Panamanian = 60,

    /// Paraguayan nationality
    Paraguayan = 61,

    /// Peruvian nationality
    Peruvian = 62,

    /// Polish nationality
    Polish = 63,

    /// Portuguese nationality
    Portuguese = 64,

    /// Qatari nationality
    Qatari = 65,

    /// Romanian nationality
    Romanian = 66,

    /// Salvadoran nationality
    Salvadoran = 68,

    /// Saudi nationality
    Saudi = 69,

    /// Scottish nationality
    Scottish = 70,

    /// Serbian nationality
    Serbian = 71,

    /// Singaporean nationality
    Singaporean = 72,

    /// Slovakian nationality
    Slovakian = 73,

    /// Slovenian nationality
    Slovenian = 74,

    /// South Korean nationality
    SouthKorean = 75,

    /// South African nationality
    SouthAfrican = 76,

    /// Spanish nationality
    Spanish = 77,

    /// Swedish nationality
    Swedish = 78,

    /// Swiss nationality
    Swiss = 79,

    /// Thai nationality
    Thai = 80,

    /// Turkish nationality
    Turkish = 81,

    /// Uruguayan nationality
    Uruguayan = 82,

    /// Ukrainian nationality
    Ukrainian = 83,

    /// Venezuelan nationality
    Venezuelan = 84,

    /// Barbadian nationality
    Barbadian = 85,

    /// Welsh nationality
    Welsh = 86,

    /// Vietnamese nationality
    Vietnamese = 87,

    /// Algerian nationality
    Algerian = 88,

    /// Bosnian nationality
    Bosnian = 89,

    /// Filipino nationality
    Filipino = 90,

    /// No nationality specified
    None = 255,
}
