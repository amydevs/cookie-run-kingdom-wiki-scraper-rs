use std::{str::FromStr, vec};

use regex::Regex;

use scraper::{Html, Selector, element_ref::Select, ElementRef};

use typesand::*;
use crate::{rarity::{rarity_types::{Rarity}}, tools::ClientWrapper};

pub struct CharacterTools<'a> {
    pub clientwrapper: &'a ClientWrapper,
    selectors: CharacterSelectors
}
impl<'a> CharacterTools<'a> {
    pub fn from_clientwrapper(clientwrapper: &'a ClientWrapper) -> Self {
        let inst = Self {
            clientwrapper: clientwrapper,
            selectors: CharacterSelectors::new()
        };
        return inst
    }

    pub async fn get_characters_urls(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut urls:Vec<String> = vec![];
        let document = Html::parse_document(&self.clientwrapper.client.get(format!("{}{}", &self.clientwrapper.base_url, "/wiki/List_of_Cookies")).send().await?.text().await?);
        let selector = Selector::parse(".wikitable > tbody th > a:not(.image)").unwrap();
        for element in document.select(&selector) {
            if !element.inner_html().contains("non-playable") {
                urls.push(element.value().attr("href").unwrap_or_default().to_owned());
            }
        }
        Ok(urls)
    }
    pub async fn get_character(&self, url: &String) -> Result<Character, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.clientwrapper.client.get(format!("{}{}", &self.clientwrapper.base_url, url)).send().await?.text().await?);
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
            rarity: Rarity::from_str(document.select(&self.selectors.rarity).next().unwrap().value().attr("alt").unwrap().replace("\"", "").as_str()).ok(),
            position: CharacterPos::from_str(temppos.as_str()).ok()
        };
        Ok(characterinst)
    }

    pub async fn get_treasures(&self) -> Result<Vec<Treasure>, Box<dyn std::error::Error>> {
        let mut temptreasures: Vec<Treasure> = vec![];
        let document = Html::parse_document(&self.clientwrapper.client.get(format!("{}{}", &self.clientwrapper.base_url, "/wiki/Treasures")).send().await?.text().await?);
        let headerelesel = Selector::parse("table[border='0']").unwrap();
        let headersel = Selector::parse(".mw-headline").unwrap();
        let sel = Selector::parse("th[style='text-align:left']").unwrap();
        
        for header in document.select(&headerelesel) {
            let raritytype = header.select(&headersel).next().unwrap().value().attr("id").unwrap().replace("_Treasures", "");
            
            let tablediv = ElementRef::wrap(header.next_sibling().unwrap().next_sibling().unwrap()).unwrap();
            for e in tablediv.select(&sel) {
                let first_child = e.first_child().unwrap().value().as_element().unwrap();
                temptreasures.push(Treasure {
                    name: first_child.attr("title").unwrap().to_owned(),
                    image_path: Regex::new(r"/revision/.*").unwrap().replace(first_child.attr("href").unwrap(), "").to_string(),
                    rarity: Rarity::from_str(raritytype.as_str()).unwrap()
                })
            }
        }
        Ok(temptreasures)
    }
    
}

pub mod typesand {
    use crate::rarity::{rarity_types::{Rarity}};

    use serde::{Serialize, Deserialize};
    use strum_macros::EnumString;
    use scraper::Selector;
    use ts_rs::TS;

    #[cfg(feature = "enum-u8")]
    use serde_repr::*;

    #[derive(Serialize, Deserialize, TS)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct Character {
        pub name: String,
        pub r#type: Option<CharacterType>,
        pub image_path: String,
        pub rarity: Option<Rarity>,
        pub position: Option<CharacterPos>
    }

    #[cfg_attr(not(feature = "enum-u8"), derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "enum-u8", derive(Serialize_repr, Deserialize_repr), repr(u8))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(TS, EnumString)]
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
    
    #[cfg_attr(not(feature = "enum-u8"), derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "enum-u8", derive(Serialize_repr, Deserialize_repr), repr(u8))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(TS, EnumString)]
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

    #[derive(Serialize, Deserialize, TS)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct Treasure {
        pub name: String,
        pub image_path: String,
        pub rarity: Rarity
    }
}