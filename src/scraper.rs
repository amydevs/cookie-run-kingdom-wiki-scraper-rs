use std::{str::FromStr};

use regex::Regex;

use scraper::{Html, Selector};

use TScraper::{Character, CharacterRarity, CharacterSelectors, CharacterType};

pub struct Scraper {
    base_url: String,
    pub client: reqwest::Client,
    selectors: CharacterSelectors
}
impl Scraper {
    pub fn new() -> Self {
        let inst = Self {
            base_url: "https://cookierunkingdom.fandom.com".to_owned(),
            client: reqwest::Client::new(),
            selectors: CharacterSelectors::new()
        };
        return inst
    }
    pub async fn get_characters_urls(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut urls:Vec<String> = vec![];
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, "/wiki/List_of_Cookies")).send().await?.text().await?);
        let selector = Selector::parse(".wikitable > tbody th > a:not(.image)").unwrap();
        for element in document.select(&selector) {
            
            urls.push(element.value().attr("href").unwrap_or_default().to_owned());
        }
        Ok(urls)
    }
    pub async fn get_character(&self, url: &String) -> Result<Character, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, url)).send().await?.text().await?);
        let mut temptype = document.select(&self.selectors.r#type).last().unwrap().text().collect::<String>();
        temptype.remove(0);
        let characterinst = Character {
            name: document.select(&self.selectors.name)
                .next().unwrap().inner_html().replace("\t", "").replace("\n", ""),
            r#type: CharacterType::from_str(temptype.as_str()).unwrap_or(CharacterType::Null),
            image_path: Regex::new(r"/revision/.*").unwrap().replace(document.select(&self.selectors.imagepath)
            .next().unwrap().value().attr("src").unwrap(), "").to_string(),
            rarity: CharacterRarity::from_str(document.select(&self.selectors.rarity)
            .next().unwrap().value().attr("alt").unwrap().replace("\"", "").as_str()).unwrap_or(CharacterRarity::Null)
        };
        Ok(characterinst)
    }
}

pub mod TScraper {
    use serde::{Serialize, Deserialize};
    use serde_repr::*;
    use strum_macros::EnumString;
    use scraper::Selector;

    
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Character {
        pub name: String,
        pub r#type: CharacterType,
        pub image_path: String,
        pub rarity: CharacterRarity
    }

    #[derive(Serialize_repr, Deserialize_repr, Debug, EnumString)]
    #[repr(u8)]
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

    #[derive(Serialize_repr, Deserialize_repr, Debug, EnumString)]
    #[repr(u8)]
    pub enum CharacterRarity {
        Special,
        Common,
        Rare,
        Epic,
        Legendary,
        Ancient,
        Null
    }

    #[derive(Debug)]
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
}