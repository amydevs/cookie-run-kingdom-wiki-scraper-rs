use scraper::Selector;
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

pub struct CharacterSelectors {
    pub name: Selector,
    pub r#type: Selector,
    pub imagepath: Selector,
    pub rarity: Selector
}
impl CharacterSelectors {
    pub fn new() -> Self {
        CharacterSelectors {
            name: Selector::parse(".page-header__title#firstHeading").unwrap(),
            r#type: Selector::parse("[data-source='role']").unwrap(),
            imagepath: Selector::parse(".pi-image-thumbnail").unwrap(),
            rarity: Selector::parse("[data-source='rarity'] img").unwrap()
        }
    }
}

