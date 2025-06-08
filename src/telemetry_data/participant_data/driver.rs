//! Defines the `Driver` enum representing all possible drivers in the game.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Enum representing all possible drivers in the game
#[derive(Deserialize_repr, Debug, Serialize_repr, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Driver {
    /// Driver: Carlos Sainz
    #[serde(rename = "Carlos Sainz")]
    CarlosSainz = 0,

    /// Driver: Daniil Kvyat
    #[serde(rename = "Daniil Kvyat")]
    DaniilKvyat = 1,

    /// Driver: Daniel Ricciardo
    #[serde(rename = "Daniel Ricciardo")]
    DanielRicciardo = 2,

    /// Driver: Fernando Alonso
    #[serde(rename = "Fernando Alonso")]
    FernandoAlonso = 3,

    /// Driver: Felipe Massa
    #[serde(rename = "Felipe Massa")]
    FelipeMassa = 4,

    /// Driver: Kimi Raikkonen
    #[serde(rename = "Kimi Raikkonen")]
    KimiRaikkonen = 6,

    /// Driver: Lewis Hamilton
    #[serde(rename = "Lewis Hamilton")]
    LewisHamilton = 7,

    /// Driver: Max Verstappen
    #[serde(rename = "Max Verstappen")]
    MaxVerstappen = 9,

    /// Driver: Nico Hulkenburg
    #[serde(rename = "Nico Hulkenburg")]
    NicoHulkenburg = 10,

    /// Driver: Kevin Magnussen
    #[serde(rename = "Kevin Magnussen")]
    KevinMagnussen = 11,

    /// Driver: Romain Grosjean
    #[serde(rename = "Romain Grosjean")]
    RomainGrosjean = 12,

    /// Driver: Sebastian Vettel
    #[serde(rename = "Sebastian Vettel")]
    SebastianVettel = 13,

    /// Driver: Sergio Perez
    #[serde(rename = "Sergio Perez")]
    SergioPerez = 14,

    /// Driver: Valtteri Bottas
    #[serde(rename = "Valtteri Bottas")]
    ValtteriBottas = 15,

    /// Driver: Esteban Ocon
    #[serde(rename = "Esteban Ocon")]
    EstebanOcon = 17,

    /// Driver: Lance Stroll
    #[serde(rename = "Lance Stroll")]
    LanceStroll = 19,

    /// Driver: Arron Barnes
    #[serde(rename = "Arron Barnes")]
    ArronBarnes = 20,

    /// Driver: Martin Giles
    #[serde(rename = "Martin Giles")]
    MartinGiles = 21,

    /// Driver: Alex Murray
    #[serde(rename = "Alex Murray")]
    AlexMurray = 22,

    /// Driver: Lucas Roth
    #[serde(rename = "Lucas Roth")]
    LucasRoth = 23,

    /// Driver: Igor Correia
    #[serde(rename = "Igor Correia")]
    IgorCorreia = 24,

    /// Driver: Sophie Levasseur
    #[serde(rename = "Sophie Levasseur")]
    SophieLevasseur = 25,

    /// Driver: Jonas Schiffer
    #[serde(rename = "Jonas Schiffer")]
    JonasSchiffer = 26,

    /// Driver: Alain Forest
    #[serde(rename = "Alain Forest")]
    AlainForest = 27,

    /// Driver: Jay Letourneau
    #[serde(rename = "Jay Letourneau")]
    JayLetourneau = 28,

    /// Driver: Esto Saari
    #[serde(rename = "Esto Saari")]
    EstoSaari = 29,

    /// Driver: Yasar Atiyeh
    #[serde(rename = "Yasar Atiyeh")]
    YasarAtiyeh = 30,

    /// Driver: Callisto Calabresi
    #[serde(rename = "Callisto Calabresi")]
    CallistoCalabresi = 31,

    /// Driver: Naota Izum
    #[serde(rename = "Naota Izum")]
    NaotaIzum = 32,

    /// Driver: Howard Clarke
    #[serde(rename = "Howard Clarke")]
    HowardClarke = 33,

    /// Driver: Lars Kaufmann
    #[serde(rename = "Lars Kaufmann")]
    LarsKaufmann = 34,

    /// Driver: Marie Laursen
    #[serde(rename = "Marie Laursen")]
    MarieLaursen = 35,

    /// Driver: Flavio Nieves
    #[serde(rename = "Flavio Nieves")]
    FlavioNieves = 36,

    /// Driver: Peter Belousov
    #[serde(rename = "Peter Belousov")]
    PeterBelousov = 37,

    /// Driver: Klimek Michalski
    #[serde(rename = "Klimek Michalski")]
    KlimekMichalski = 38,

    /// Driver: Santiago Moreno
    #[serde(rename = "Santiago Moreno")]
    SantiagoMoreno = 39,

    /// Driver: Benjamin Coppens
    #[serde(rename = "Benjamin Coppens")]
    BenjaminCoppens = 40,

    /// Driver: Noah Visser
    #[serde(rename = "Noah Visser")]
    NoahVisser = 41,

    /// Driver: Gert Waldmuller
    #[serde(rename = "Gert Waldmuller")]
    GertWaldmuller = 42,

    /// Driver: Julian Quesada
    #[serde(rename = "Julian Quesada")]
    JulianQuesada = 43,

    /// Driver: Daniel Jones
    #[serde(rename = "Daniel Jones")]
    DanielJones = 44,

    /// Driver: Artem Markelov
    #[serde(rename = "Artem Markelov")]
    ArtemMarkelov = 45,

    /// Driver: Tadasuke Makino
    #[serde(rename = "Tadasuke Makino")]
    TadasukeMakino = 46,

    /// Driver: Sean Gelael
    #[serde(rename = "Sean Gelael")]
    SeanGelael = 47,

    /// Driver: Nyck De Vries
    #[serde(rename = "Nyck De Vries")]
    NyckDeVries = 48,

    /// Driver: Jack Aitken
    #[serde(rename = "Jack Aitken")]
    JackAitken = 49,

    /// Driver: George Russell
    #[serde(rename = "George Russell")]
    GeorgeRussell = 50,

    /// Driver: Maximilian Gunther
    #[serde(rename = "Maximilian Gunther")]
    MaximilianGunther = 51,

    /// Driver: Nirei Fukuzumi
    #[serde(rename = "Nirei Fukuzumi")]
    NireiFukuzumi = 52,

    /// Driver: Luca Ghiotto
    #[serde(rename = "Luca Ghiotto")]
    LucaGhiotto = 53,

    /// Driver: Lando Norris
    #[serde(rename = "Lando Norris")]
    LandoNorris = 54,

    /// Driver: Sergio Sette Camara
    #[serde(rename = "Sergio Sette Camara")]
    SergioSetteCamara = 55,

    /// Driver: Louis Deletraz
    #[serde(rename = "Louis Deletraz")]
    LouisDeletraz = 56,

    /// Driver: Antonio Fuoco
    #[serde(rename = "Antonio Fuoco")]
    AntonioFuoco = 57,

    /// Driver: Charles Leclerc
    #[serde(rename = "Charles Leclerc")]
    CharlesLeclerc = 58,

    /// Driver: Pierre Gasly
    #[serde(rename = "Pierre Gasly")]
    PierreGasly = 59,

    /// Driver: Alexander Albon
    #[serde(rename = "Alexander Albon")]
    AlexanderAlbon = 62,

    /// Driver: Nicholas Latifi
    #[serde(rename = "Nicholas Latifi")]
    NicholasLatifi = 63,

    /// Driver: Dorian Boccolacci
    #[serde(rename = "Dorian Boccolacci")]
    DorianBoccolacci = 64,

    /// Driver: Niko Kari
    #[serde(rename = "Niko Kari")]
    NikoKari = 65,

    /// Driver: Roberto Merhi
    #[serde(rename = "Roberto Merhi")]
    RobertoMerhi = 66,

    /// Driver: Arjun Maini
    #[serde(rename = "Arjun Maini")]
    ArjunMaini = 67,

    /// Driver: Alessio Lorandi
    #[serde(rename = "Alessio Lorandi")]
    AlessioLorandi = 68,

    /// Driver: Ruben Meijer
    #[serde(rename = "Ruben Meijer")]
    RubenMeijer = 69,

    /// Driver: Rashid Nair
    #[serde(rename = "Rashid Nair")]
    RashidNair = 70,

    /// Driver: Jack Tremblay
    #[serde(rename = "Jack Tremblay")]
    JackTremblay = 71,

    /// Driver: Devon Butler
    #[serde(rename = "Devon Butler")]
    DevonButler = 72,

    /// Driver: Lukas Weber
    #[serde(rename = "Lukas Weber")]
    LukasWeber = 73,

    /// Driver: Antonio Giovinazzi
    #[serde(rename = "Antonio Giovinazzi")]
    AntonioGiovinazzi = 74,

    /// Driver: Robert Kubica
    #[serde(rename = "Robert Kubica")]
    RobertKubica = 75,

    /// Driver: Alain Prost
    #[serde(rename = "Alain Prost")]
    AlainProst = 76,

    /// Driver: Ayrton Senna
    #[serde(rename = "Ayrton Senna")]
    AyrtonSenna = 77,

    /// Driver: Nobuharu Matsushita
    #[serde(rename = "Nobuharu Matsushita")]
    NobuharuMatsushita = 78,

    /// Driver: Nikita Mazepin
    #[serde(rename = "Nikita Mazepin")]
    NikitaMazepin = 79,

    /// Driver: Guanyu Zhou
    #[serde(rename = "Guanyu Zhou")]
    GuanyuZhou = 80,

    /// Driver: Mick Schumacher
    #[serde(rename = "Mick Schumacher")]
    MickSchumacher = 81,

    /// Driver: Callum Ilott
    #[serde(rename = "Callum Ilott")]
    CallumIlott = 82,

    /// Driver: Juan Manuel Correa
    #[serde(rename = "Juan Manuel Correa")]
    JuanManuelCorrea = 83,

    /// Driver: Jordan King
    #[serde(rename = "Jordan King")]
    JordanKing = 84,

    /// Driver: Mahaveer Raghunathan
    #[serde(rename = "Mahaveer Raghunathan")]
    MahaveerRaghunathan = 85,

    /// Driver: Tatiana Calderon
    #[serde(rename = "Tatiana Calderon")]
    TatianaCalderon = 86,

    /// Driver: Anthoine Hubert
    #[serde(rename = "Anthoine Hubert")]
    AnthoineHubert = 87,

    /// Driver: Guiliano Alesi
    #[serde(rename = "Guiliano Alesi")]
    GuilianoAlesi = 88,

    /// Driver: Ralph Boschung
    #[serde(rename = "Ralph Boschung")]
    RalphBoschung = 89,

    /// Driver: Michael Schumacher
    #[serde(rename = "Michael Schumacher")]
    MichaelSchumacher = 90,

    /// Driver: Dan Ticktum
    #[serde(rename = "Dan Ticktum")]
    DanTicktum = 91,

    /// Driver: Marcus Armstrong
    #[serde(rename = "Marcus Armstrong")]
    MarcusArmstrong = 92,

    /// Driver: Christian Lundgaard
    #[serde(rename = "Christian Lundgaard")]
    ChristianLundgaard = 93,

    /// Driver: Yuki Tsunoda
    #[serde(rename = "Yuki Tsunoda")]
    YukiTsunoda = 94,

    /// Driver: Jehan Daruvala
    #[serde(rename = "Jehan Daruvala")]
    JehanDaruvala = 95,

    /// Driver: Gulherme Samaia
    #[serde(rename = "Gulherme Samaia")]
    GulhermeSamaia = 96,

    /// Driver: Pedro Piquet
    #[serde(rename = "Pedro Piquet")]
    PedroPiquet = 97,

    #[serde(rename = "Felipe Drugovich")]
    FelipeDrugovich = 98,

    #[serde(rename = "Robert Schwartzman")]
    RobertSchwartzman = 99,

    #[serde(rename = "Roy Nissany")]
    RoyNissany = 100,

    #[serde(rename = "Marino Sato")]
    MarinoSato = 101,

    #[serde(rename = "Aidan Jackson")]
    AidanJackson = 102,

    #[serde(rename = "Casper Akkerman")]
    CasperAkkerman = 103,

    #[serde(rename = "Jenson Button")]
    JensonButton = 109,

    #[serde(rename = "David Coulthard")]
    DavidCoulthard = 110,

    #[serde(rename = "Nico Rosberg")]
    NicoRosberg = 111,

    #[serde(rename = "Oscar Piastri")]
    OscarPiastri = 112,

    #[serde(rename = "Liam Lawson")]
    LiamLawson = 113,

    #[serde(rename = "Juri Vips")]
    JuriVips = 114,

    #[serde(rename = "Theo Pourchaire")]
    TheoPourchaire = 115,

    #[serde(rename = "Richard Verschoor")]
    RichardVerschoor = 116,

    #[serde(rename = "Lirim Zendeli")]
    LirimZendeli = 117,

    #[serde(rename = "David Beckmann")]
    DavidBeckmann = 118,

    #[serde(rename = "Alessio Deledda")]
    AlessioDeledda = 121,

    #[serde(rename = "Bent Viscaal")]
    BentViscaal = 122,

    #[serde(rename = "Enzo Fittipaldi")]
    EnzoFittipaldi = 123,

    #[serde(rename = "Mark Webber")]
    MarkWebber = 125,

    #[serde(rename = "Jacques Villeneuve")]
    JacquesVilleneuve = 126,

    #[serde(rename = "Callie Mayer")]
    CallieMayer = 127,

    #[serde(rename = "Noah Bell")]
    NoahBell = 128,

    #[serde(rename = "Jake Hughes")]
    JakeHughes = 129,

    #[serde(rename = "Frederik Vesti")]
    FrederikVesti = 130,

    #[serde(rename = "Olli Caldwell")]
    OlliCaldwell = 131,

    #[serde(rename = "Logan Sargeant")]
    LoganSargeant = 132,

    #[serde(rename = "Cem Bolukbasi")]
    CemBolukbasi = 133,

    #[serde(rename = "Ayumu Iwasa")]
    AyumuIwasa = 134,

    #[serde(rename = "Clement Novalak")]
    ClementNovalak = 135,

    #[serde(rename = "Jack Doohan")]
    JackDoohan = 136,

    #[serde(rename = "Amaury Cordeel")]
    AmauryCordeel = 137,

    #[serde(rename = "Dennis Hauger")]
    DennisHauger = 138,

    #[serde(rename = "Calan Williams")]
    CalanWilliams = 139,

    #[serde(rename = "Jamie Chadwick")]
    JamieChadwick = 140,

    #[serde(rename = "Kamui Kobayashi")]
    KamuiKobayashi = 141,

    #[serde(rename = "Pastor Maldonado")]
    PastorMaldonado = 142,

    #[serde(rename = "Mika Hakkinen")]
    MikaHakkinen = 143,

    #[serde(rename = "Nigel Mansell")]
    NigelMansell = 144,

    #[serde(rename = "Zane Maloney")]
    ZaneMaloney = 145,

    #[serde(rename = "Victor Martins")]
    VictorMartins = 146,

    #[serde(rename = "Oliver Bearman")]
    OliverBearman = 147,

    #[serde(rename = "Jak Crawford")]
    JakCrawford = 148,

    #[serde(rename = "Isack Hadjar")]
    IsackHadjar = 149,

    #[serde(rename = "Arthur Leclerc")]
    ArthurLeclerc = 150,

    #[serde(rename = "Brad Benavides")]
    BradBenavides = 151,

    #[serde(rename = "Roman Stanek")]
    RomanStanek = 152,

    #[serde(rename = "Kush Maini")]
    KushMaini = 153,

    #[serde(rename = "James Hunt")]
    JamesHunt = 154,

    #[serde(rename = "Juan Pablo Montoya")]
    JuanPabloMontoya = 155,

    #[serde(rename = "Brendon Leigh")]
    BrendonLeigh = 156,

    #[serde(rename = "David Tonizza")]
    DavidTonizza = 157,

    #[serde(rename = "Jarno Opmeer")]
    JarnoOpmeer = 158,

    #[serde(rename = "Lucas Blakeley")]
    LucasBlakeley = 159,

    #[serde(other)]
    Unknown = 255,

    /// Driver: Joshua Durksen
    #[serde(rename = "Joshua Durksen")]
    JoshuaDurksen = 164,

    /// Driver: Andrea Kimi Antonelli
    #[serde(rename = "Andrea Kimi Antonelli")]
    AndreaKimiAntonelli = 165,

    /// Driver: Ritomo Miyata
    #[serde(rename = "Ritomo Miyata")]
    RitomoMiyata = 166,

    /// Driver: Rafael Villagomez
    #[serde(rename = "Rafael Villagomez")]
    RafaelVillagomez = 167,

    /// Driver: Zak O'Sullivan
    #[serde(rename = "Zak O'Sullivan")]
    ZakOSullivan = 168,

    /// Driver: Pepe Marti
    #[serde(rename = "Pepe Marti")]
    PepeMarti = 169,

    /// Driver: Sonny Hayes
    #[serde(rename = "Sonny Hayes")]
    SonnyHayes = 170,

    /// Driver: Joshua Pearce
    #[serde(rename = "Joshua Pearce")]
    JoshuaPearce = 171,

    /// Driver: Callum Voisin
    #[serde(rename = "Callum Voisin")]
    CallumVoisin = 172,

    /// Driver: Matias Zagazeta
    #[serde(rename = "Matias Zagazeta")]
    MatiasZagazeta = 173,

    /// Driver: Nikola Tsolov
    #[serde(rename = "Nikola Tsolov")]
    NikolaTsolov = 174,

    /// Driver: Tim Tramnitz
    #[serde(rename = "Tim Tramnitz")]
    TimTramnitz = 175,

    /// Driver: Luca Cortez
    #[serde(rename = "Luca Cortez")]
    LucaCortez = 185,

    /// Driver: Paul Aron
    #[serde(rename = "Paul Aron")]
    PaulAron = 160,

    /// Driver: Gabriel Bortoleto
    #[serde(rename = "Gabriel Bortoleto")]
    GabrielBortoleto = 161,

    /// Driver: Franco Colapinto
    #[serde(rename = "Franco Colapinto")]
    FrancoColapinto = 162,

    /// Driver: Taylor Barnard
    #[serde(rename = "Taylor Barnard")]
    TaylorBarnard = 163,
}
