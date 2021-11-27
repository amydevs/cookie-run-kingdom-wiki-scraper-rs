use std::{str::FromStr, vec};

use regex::Regex;

use scraper::{Html, Selector, element_ref::Select};

use Typesand::*;

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

    pub async fn get_treasures(&self) -> Result<Vec<Treasure>, Box<dyn std::error::Error>> {
        let mut temptreasures: Vec<Treasure> = vec![];
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, "/wiki/Treasures")).send().await?.text().await?);
        let sel = Selector::parse("th[style='text-align:left']").unwrap();
        for e in document.select(&sel) {
            let first_child = e.first_child().unwrap().value().as_element().unwrap();
            temptreasures.push(Treasure {
                name: first_child.attr("title").unwrap().to_owned(),
                image_path: Regex::new(r"/revision/.*").unwrap().replace(first_child.attr("href").unwrap(), "").to_string()
            })
        }
        Ok(temptreasures)
    }
    
}

pub mod Typesand {
    use serde::{Serialize, Deserialize};
    use strum_macros::EnumString;
    use scraper::Selector;
    use ts_rs::TS;

    #[cfg(feature = "use-repr")]
    use serde_repr::*;

    #[derive(Serialize, Deserialize, TS, Debug)]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct Character {
        pub name: String,
        pub r#type: Option<CharacterType>,
        pub image_path: String,
        pub rarity: Option<CharacterRarity>,
        pub position: Option<CharacterPos>
    }

    #[cfg_attr(not(feature = "use-repr"), derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "use-repr", derive(Serialize_repr, Deserialize_repr), repr(u8))]
    #[derive(TS, Debug, EnumString)]
    #[ts(export)]
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

    #[cfg_attr(not(feature = "use-repr"), derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "use-repr", derive(Serialize_repr, Deserialize_repr), repr(u8))]
    #[derive(TS, Debug, EnumString)]
    #[ts(export)]
    pub enum CharacterRarity {
        Special,
        Common,
        Rare,
        Epic,
        Legendary,
        Ancient
    }

    
    #[cfg_attr(not(feature = "use-repr"), derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "use-repr", derive(Serialize_repr, Deserialize_repr), repr(u8))]
    #[derive(TS, Debug, EnumString)]
    #[ts(export)]
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

    #[derive(Serialize, Deserialize, TS, Debug)]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct RarityChances {
        pub rarity: Option<CharacterRarity>,
        pub cookie: f32,
        pub soulstone: f32
    }

    #[derive(Serialize, Deserialize, TS, Debug)]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct Treasure {
        pub name: String,
        pub image_path: String,
    }
}