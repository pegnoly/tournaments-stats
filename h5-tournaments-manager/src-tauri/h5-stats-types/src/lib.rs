use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, EnumIter, FromRepr, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum RaceType {
    NotDetected = 0,
    Heaven = 1,
    Inferno = 2,
    Necropolis = 3,
    Preserve = 4, 
    Dungeon = 5, 
    Academy = 6, 
    Fortress = 7,
    Stronghold = 8
}

#[derive(Debug, Serialize, Deserialize, EnumIter, FromRepr, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum HeroType {
    NotDetected = 0,
    Orrin = 1,
    Mardigo = 2,
    Nathaniel = 3,
    Maeve = 4,
    Brem = 5,
    Sarge = 6,
    Christian = 7,
    Ving = 8,
    Oddrema = 9,
    Nymus = 10,
    Calid = 11,
    Deleb = 12,
    Grok = 13,
    Marder = 14,
    Efion = 15,
    Jazaz = 16,
    Gles = 17,
    Nemor = 18,
    Aberrar = 19,
    Tamika = 20,
    Pelt = 21,
    Straker = 22,
    Muscip = 23,
    Effig = 24,
    Metlirn = 25,
    Nadaur = 26,
    Diraya = 27,
    Elleshar = 28,
    Ossir = 29,
    Gillion = 30,
    Itil = 31,
    Linaas = 32,
    Almegir = 33,
    Urunir = 34,
    Menel = 35,
    Eruina = 36,
    Dalom = 37,
    Ferigl = 38,
    Ohtarig = 39,
    Inagost = 40,
    Tan = 41,
    Astral = 42,
    Havez = 43,
    Faiz = 44,
    Isher = 45,
    Razzak = 46,
    Nur = 47,
    Sufi = 48,
    Ingvar = 49,
    Bersy = 50,
    Skeggy = 51,
    Brand = 52,
    Ottar = 53,
    Egil = 54,
    Una = 55,
    Vegeyr = 56,
    Hero1 = 57,
    Hero2 = 58,
    Hero3 = 59,
    Hero4 = 60,
    Hero6 = 61,
    Hero7 = 62,
    Hero8 = 63,
    Hero9 = 64
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Race {
    pub id: RaceType,
    pub actual_name: String
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hero {
    pub id: HeroType,
    pub race: RaceType,
    pub actual_name: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Tournament {
    pub id: Uuid,
    pub server_id: i64,
    pub channel_id: i64,
    pub name: String
}


/// A match between two players in a concrete tournament. Contains Games.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Match {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub first_player: String,
    pub second_player: String,
    pub message: u64
}

/// Possible game outcomes
#[derive(Debug, Serialize, Deserialize, FromRepr, Clone, PartialEq, Eq)]
#[repr(i16)]
pub enum GameResult {
    NotDetected = 0,
    FirstPlayerWon = 1,
    SecondPlayerWon = 2
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameResultModel {
    pub id: GameResult,
    pub name: String
}

/// Possible colors used in bargains
#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromRepr, PartialEq, Eq)]
#[repr(i16)]
pub enum BargainsColor {
    NotDetected,
    ColorRed,
    ColorBlue
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BargainsColorModel {
    pub id: BargainsColor,
    pub name: String
}

/// A single game between two players.
#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: Uuid,
    pub match_id: Uuid,
    pub first_player_race: RaceType,
    pub first_player_hero: HeroType,
    pub second_player_race: RaceType,
    pub second_player_hero: HeroType,
    pub bargains_color: BargainsColor,
    pub bargains_amount: i16,
    pub result: GameResult
}

impl Default for Game {
    fn default() -> Self {
        Game {
            first_player_race: RaceType::NotDetected,
            first_player_hero: HeroType::NotDetected,
            second_player_race: RaceType::NotDetected,
            second_player_hero: HeroType::NotDetected,
            bargains_color: BargainsColor::NotDetected,
            result: GameResult::NotDetected,
            id: uuid::Uuid::new_v4(),
            match_id: uuid::Uuid::default(),
            bargains_amount: 0
        }
    }
}