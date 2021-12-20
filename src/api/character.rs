use std::{str::FromStr, vec};

use regex::Regex;

use scraper::{Html, Selector};

use character_types::*;
use crate::{api::rarity::{rarity_types::{Rarity}}, tools::ClientWrapper};

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
            illustration_img_path: Regex::new(r"/revision/.*").unwrap().replace(document.select(&self.selectors.illustration_img_path)
            .next().unwrap().value().attr("src").unwrap(), "").to_string(),
            soulstone_img_path: document.select(&self.selectors.soulstone_img_path).next().unwrap().value().attr("data-src").unwrap().to_owned(),
            rarity: Rarity::from_str(document.select(&self.selectors.rarity).next().unwrap().value().attr("alt").unwrap().replace("\"", "").as_str()).ok(),
            position: CharacterPos::from_str(temppos.as_str()).ok()
        };
        Ok(characterinst)
    }
}

pub mod character_types {
    use crate::api::rarity::{rarity_types::{Rarity}};

    use serde::{Serialize, Deserialize};
    use strum_macros::EnumString;
    use scraper::{Selector, element_ref::Select};
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
        pub illustration_img_path: String,
        pub soulstone_img_path: String,
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
        pub illustration_img_path: Selector,
        pub soulstone_img_path: Selector,
        pub rarity: Selector,
        pub position: Selector
    }
    impl CharacterSelectors {
        pub fn new() -> Self {
            CharacterSelectors {
                name: Selector::parse(".page-header__title#firstHeading").unwrap(),
                r#type: Selector::parse("[data-source='role']").unwrap(),
                illustration_img_path: Selector::parse(".pi-image-thumbnail").unwrap(),
                soulstone_img_path: Selector::parse("img[data-image-name*='Soulstone']").unwrap(),
                rarity: Selector::parse("[data-source='rarity'] img").unwrap(),
                position: Selector::parse("td[data-source='position']").unwrap()
            }
        }
    }
}