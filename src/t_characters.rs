use serde::{Serialize, Deserialize};
use strum_macros::EnumString;


#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub name: String,
    pub r#type: CharacterType,
    pub imagepath: String,
    
    pub rarity: CharacterRarity
}

#[derive(Serialize, Deserialize, Debug, EnumString)]
pub enum CharacterType {
    Ambush,
    Bomber,
    Charge,
    Defense,
    Healing,
    Magic,
    Ranged,
    Support,
    Null
}

#[derive(Serialize, Deserialize, Debug, EnumString)]
pub enum CharacterRarity {
    Special,
    Common,
    Rare,
    Epic,
    Legendary,
    Ancient,
    Null
}