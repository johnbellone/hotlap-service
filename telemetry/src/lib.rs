// SPDX-License-Identifier: Apache-2.0
// https://github.com/EmperorCookie/accapi
use std::fmt;

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LapType {
    Regular = 0,
    Outlap = 1,
    Inlap = 2,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DriverCategory {
    Bronze = 0,
    Silver = 1,
    Gold = 2,
    Platinum = 3,
    Unknown = 255,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CarLocation {
    Unknown = 0,
    Track = 1,
    Pitlane = 2,
    PitEntry = 3,
    PitExit = 4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlagType {
    None,
    Blue,
    Yellow,
    Black,
    White,
    Checkered,
    Penalty,
    Green,
    Orange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BroadcastEventType {
    Unknown,
    GreenFlag,
    SessionOver,
    PenaltyCommunication,
    Incident,
    LapCompleted,
    BestSessionLap,
    BestPersonalLap,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SessionPhase {
    Unknown = 0,
    Starting = 1,
    PreFormation = 2,
    FormationLap = 3,
    PreSession = 4,
    Session = 5,
    SessionOver = 6,
    PostSession = 7,
    ResultUI = 8,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SessionType {
    Practice = 0,
    Qualifying = 4,
    Superpole = 9,
    Race = 10,
    Hotlap = 11,
    HotStint = 12,
    HotlapSuperpole = 13,
    Replay = 14,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Nationality {
    Unknown = 0,
    Italy = 1,
    Germany = 2,
    France = 3,
    Spain = 4,
    GreatBritain = 5,
    Hungary = 6,
    Belgium = 7,
    Switzerland = 8,
    Austria = 9,
    Russia = 10,
    Thailand = 11,
    Netherlands = 12,
    Poland = 13,
    Argentina = 14,
    Monaco = 15,
    Ireland = 16,
    Brazil = 17,
    SouthAfrica = 18,
    PuertoRico = 19,
    Slovakia = 20,
    Oman = 21,
    Greece = 22,
    SaudiArabia = 23,
    Norway = 24,
    Turkey = 25,
    SouthKorea = 26,
    Lebanon = 27,
    Armenia = 28,
    Mexico = 29,
    Sweden = 30,
    Finland = 31,
    Denmark = 32,
    Croatia = 33,
    Canada = 34,
    China = 35,
    Portugal = 36,
    Singapore = 37,
    Indonesia = 38,
    USA = 39,
    NewZealand = 40,
    Australia = 41,
    SanMarino = 42,
    UAE = 43,
    Luxembourg = 44,
    Kuwait = 45,
    HongKong = 46,
    Colombia = 47,
    Japan = 48,
    Andorra = 49,
    Azerbaijan = 50,
    Bulgaria = 51,
    Cuba = 52,
    CzechRepublic = 53,
    Estonia = 54,
    Georgia = 55,
    India = 56,
    Israel = 57,
    Jamaica = 58,
    Latvia = 59,
    Lithuania = 60,
    Macau = 61,
    Malaysia = 62,
    Nepal = 63,
    NewCaledonia = 64,
    Nigeria = 65,
    NorthernIreland = 66,
    PapuaNewGuinea = 67,
    Philippines = 68,
    Qatar = 69,
    Romania = 70,
    Scotland = 71,
    Serbia = 72,
    Slovenia = 73,
    Taiwan = 74,
    Ukraine = 75,
    Venezuela = 76,
    Wales = 77,
}

impl fmt::Display for Nationality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            Nationality::Unknown => "Unknown",
            Nationality::Italy => "Italy",
            Nationality::Germany => "Germany",
            Nationality::France => "France",
            Nationality::Spain => "Spain",
            Nationality::GreatBritain => "Great Britain",
            Nationality::Hungary => "Hungary",
            Nationality::Belgium => "Belgium",
            Nationality::Switzerland => "Switzerland",
            Nationality::Austria => "Austria",
            Nationality::Russia => "Russia",
            Nationality::Thailand => "Thailand",
            Nationality::Netherlands => "Netherlands",
            Nationality::Poland => "Poland",
            Nationality::Argentina => "Argentina",
            Nationality::Monaco => "Monaco",
            Nationality::Ireland => "Ireland",
            Nationality::Brazil => "Brazil",
            Nationality::SouthAfrica => "South Africa",
            Nationality::PuertoRico => "Puerto Rico",
            Nationality::Slovakia => "Slovakia",
            Nationality::Oman => "Oman",
            Nationality::Greece => "Greece",
            Nationality::SaudiArabia => "Saudi Arabia",
            Nationality::Norway => "Norway",
            Nationality::Turkey => "Turkey",
            Nationality::SouthKorea => "South Korea",
            Nationality::Lebanon => "Lebanon",
            Nationality::Armenia => "Armenia",
            Nationality::Mexico => "Mexico",
            Nationality::Sweden => "Sweden",
            Nationality::Finland => "Finland",
            Nationality::Denmark => "Denmark",
            Nationality::Croatia => "Croatia",
            Nationality::Canada => "Canada",
            Nationality::China => "China",
            Nationality::Portugal => "Portugal",
            Nationality::Singapore => "Singapore",
            Nationality::Indonesia => "Indonesia",
            Nationality::USA => "USA",
            Nationality::NewZealand => "New Zealand",
            Nationality::Australia => "Australia",
            Nationality::SanMarino => "San Marino",
            Nationality::UAE => "UAE",
            Nationality::Luxembourg => "Luxembourg",
            Nationality::Kuwait => "Kuwait",
            Nationality::HongKong => "Hong Kong",
            Nationality::Colombia => "Colombia",
            Nationality::Japan => "Japan",
            Nationality::Andorra => "Andorra",
            Nationality::Azerbaijan => "Azerbaijan",
            Nationality::Bulgaria => "Bulgaria",
            Nationality::Cuba => "Cuba",
            Nationality::CzechRepublic => "Czech Republic",
            Nationality::Estonia => "Estonia",
            Nationality::Georgia => "Georgia",
            Nationality::India => "India",
            Nationality::Israel => "Israel",
            Nationality::Jamaica => "Jamaica",
            Nationality::Latvia => "Latvia",
            Nationality::Lithuania => "Lithuania",
            Nationality::Macau => "Macau",
            Nationality::Malaysia => "Malaysia",
            Nationality::Nepal => "Nepal",
            Nationality::NewCaledonia => "New Caledonia",
            Nationality::Nigeria => "Nigeria",
            Nationality::NorthernIreland => "Northern Ireland",
            Nationality::PapuaNewGuinea => "Papua New Guinea",
            Nationality::Philippines => "Philippines",
            Nationality::Qatar => "Qatar",
            Nationality::Romania => "Romania",
            Nationality::Scotland => "Scotland",
            Nationality::Serbia => "Serbia",
            Nationality::Slovenia => "Slovenia",
            Nationality::Taiwan => "Taiwan",
            Nationality::Ukraine => "Ukraine",
            Nationality::Venezuela => "Venezuela",
            Nationality::Wales => "Wales",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for SessionPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            SessionPhase::Unknown => "Unknown",
            SessionPhase::Starting => "Starting",
            SessionPhase::PreFormation => "Pre Formation",
            SessionPhase::FormationLap => "Formation Lap",
            SessionPhase::PreSession => "Pre Session",
            SessionPhase::Session => "Session",
            SessionPhase::SessionOver => "Session Over",
            SessionPhase::PostSession => "Post Session",
            SessionPhase::ResultUI => "Result UI",
        };
        write!(f, "{}", description)
    }
}

impl fmt::Display for LapType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            LapType::Regular => "Regular",
            LapType::Outlap => "Outlap",
            LapType::Inlap => "Inlap",
        };
        write!(f, "{}", description)
    }
}

impl fmt::Display for DriverCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            DriverCategory::Bronze => "Bronze",
            DriverCategory::Silver => "Silver",
            DriverCategory::Gold => "Gold",
            DriverCategory::Platinum => "Platinum",
            DriverCategory::Unknown => "Unknown",
        };
        write!(f, "{}", description)
    }
}

impl fmt::Display for CarLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            CarLocation::Unknown => "Unknown",
            CarLocation::Track => "Track",
            CarLocation::Pitlane => "Pitlane",
            CarLocation::PitEntry => "Pit Entry",
            CarLocation::PitExit => "Pit Exit",
        };
        write!(f, "{}", description)
    }
}

impl fmt::Display for SessionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self {
            SessionType::Practice => "Practice",
            SessionType::Qualifying => "Qualifying",
            SessionType::Superpole => "Superpole",
            SessionType::Race => "Race",
            SessionType::Hotlap => "Hotlap",
            SessionType::HotStint => "Hot Stint",
            SessionType::HotlapSuperpole => "Hotlap Superpole",
            SessionType::Replay => "Replay",
        };
        write!(f, "{}", description)
    }
}
