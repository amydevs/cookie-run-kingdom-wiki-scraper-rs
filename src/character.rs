use std::{str::FromStr};

use regex::Regex;

use scraper::{Html, Selector, element_ref::Select};

use TScraper::*;

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
    pub fn from_client(client: reqwest::Client) -> Self {
        let inst = Self {
            base_url: "https://cookierunkingdom.fandom.com".to_owned(),
            client: client,
            selectors: CharacterSelectors::new()
        };
        return inst
    }

    pub async fn get_characters_urls(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut urls:Vec<String> = vec![];
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, "/wiki/List_of_Cookies")).send().await?.text().await?);
        let selector = Selector::parse(".wikitable > tbody th > a:not(.image)").unwrap();
        for element in document.select(&selector) {
            if !element.inner_html().contains("non-playable") {
                urls.push(element.value().attr("href").unwrap_or_default().to_owned());
            }
        }
        Ok(urls)
    }
    pub async fn get_character(&self, url: &String) -> Result<Character, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, url)).send().await?.text().await?);
        let mut temptype = document.select(&self.selectors.r#type).last().unwrap().text().collect::<String>();
        temptype.remove(0);

        let mut temppos = document.select(&self.selectors.position).last().unwrap().text().collect::<String>();
        temppos.remove(0);
        
        let characterinst = Character {
            name: document.select(&self.selectors.name)
                .next().unwrap().inner_html().replace("\t", "").replace("\n", ""),
            r#type: CharacterType::from_str(temptype.as_str()).ok(),
            image_path: Regex::new(r"/revision/.*").unwrap().replace(document.select(&self.selectors.imagepath)
            .next().unwrap().value().attr("src").unwrap(), "").to_string(),
            rarity: CharacterRarity::from_str(document.select(&self.selectors.rarity).next().unwrap().value().attr("alt").unwrap().replace("\"", "").as_str()).ok(),
            position: CharacterPos::from_str(temppos.as_str()).ok()
        };
        Ok(characterinst)
    }


    pub async fn get_rarity_chances(&self) -> Result<Vec<RarityChances>, Box<dyn std::error::Error>> {
        let mut rarities:Vec<RarityChances> = vec![];

        let thsel = Selector::parse("th").unwrap();

        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, "/wiki/Gacha")).send().await?.text().await?);
        let table = document.select(&Selector::parse(".mw-parser-output > .wikitable").unwrap()).last().unwrap();

        for (i, ele) in table.select(&Selector::parse("tr").unwrap()).enumerate() {
            if i != 0 {
                let mut selth = ele.select(&thsel);
                rarities.push(RarityChances {
                    rarity: CharacterRarity::from_str(selth.next().unwrap().first_child().unwrap().value().as_element().unwrap().attr("title").unwrap().replace(" Cookie", "").as_str()).ok(),
                    cookie: self.getf32fromsel(&mut selth),
                    soulstone: self.getf32fromsel(&mut selth)
                })
            }
            
        }
        Ok(rarities)
    }
    fn getf32fromsel(&self, s: &mut Select) -> f32 {
        s.next().unwrap().inner_html().replace("\n", "").replace("%", "").parse().unwrap_or(0.0)
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
        pub r#type: Option<CharacterType>,
        pub image_path: String,
        pub rarity: Option<CharacterRarity>,
        pub position: Option<CharacterPos>
    }

    #[derive(Serialize_repr, Deserialize_repr, Debug, EnumString)]
    #[repr(u8)]
// #[derive(Serialize, Deserialize, Debug, EnumString)]
    pub enum CharacterType {
        Ambush,
        Bomber,
        Charge,
        Defense,
        Healing,
        Magic,
        Ranged,
        Support
    }

    #[derive(Serialize_repr, Deserialize_repr, Debug, EnumString)]
    #[repr(u8)]
// #[derive(Serialize, Deserialize, Debug, EnumString)]
    pub enum CharacterRarity {
        Special,
        Common,
        Rare,
        Epic,
        Legendary,
        Ancient
    }

    #[derive(Serialize_repr, Deserialize_repr, Debug, EnumString)]
    #[repr(u8)]
// #[derive(Serialize, Deserialize, Debug, EnumString)]
    pub enum CharacterPos {
        Rear,
        Middle,
        Front
    }

    #[derive(Debug)]
    pub struct CharacterSelectors {
        pub name: Selector,
        pub r#type: Selector,
        pub imagepath: Selector,
        pub rarity: Selector,
        pub position: Selector
    }
    impl CharacterSelectors {
        pub fn new() -> Self {
            CharacterSelectors {
                name: Selector::parse(".page-header__title#firstHeading").unwrap(),
                r#type: Selector::parse("[data-source='role']").unwrap(),
                imagepath: Selector::parse(".pi-image-thumbnail").unwrap(),
                rarity: Selector::parse("[data-source='rarity'] img").unwrap(),
                position: Selector::parse("td[data-source='position']").unwrap()
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RarityChances {
        pub rarity: Option<CharacterRarity>,
        pub cookie: f32,
        pub soulstone: f32
    }
}