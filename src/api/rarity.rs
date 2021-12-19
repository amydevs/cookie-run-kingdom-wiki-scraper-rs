use std::{str::FromStr, vec};

use regex::Regex;

use crate::tools::*;
use scraper::{Html, Selector, element_ref::Select, ElementRef};

use rarity_types::*;

pub struct RarityTools<'a> {
    pub clientwrapper: &'a ClientWrapper,
}

impl<'a> RarityTools<'a> {
    pub fn from_clientwrapper(clientwrapper: &'a ClientWrapper) -> Self {
        let inst = Self {
            clientwrapper: clientwrapper,
        };
        return inst
    }
    pub async fn get_rarity_chances(&self) -> Result<Vec<RarityChances>, Box<dyn std::error::Error>> {
        let mut rarities:Vec<RarityChances> = vec![];

        let thsel = Selector::parse("th").unwrap();

        let document = Html::parse_document(&self.clientwrapper.client.get(format!("{}{}", &self.clientwrapper.base_url, "/wiki/Gacha")).send().await?.text().await?);
        let table = document.select(&Selector::parse(".mw-parser-output > .wikitable").unwrap()).last().unwrap();

        for (i, ele) in table.select(&Selector::parse("tr").unwrap()).enumerate() {
            if i != 0 {
                let mut selth = ele.select(&thsel);
                rarities.push(RarityChances {
                    rarity: Rarity::from_str(selth.next().unwrap().first_child().unwrap().value().as_element().unwrap().attr("title").unwrap().replace(" Cookie", "").as_str()).ok(),
                    cookie: getf32fromsel(&mut selth),
                    soulstone: getf32fromsel(&mut selth)
                })
            }
            
        }
        Ok(rarities)
    }
}

pub mod rarity_types {
    use serde::{Serialize, Deserialize};
    use strum_macros::EnumString;
    use ts_rs::TS;

    #[cfg(feature = "enum-u8")]
    use serde_repr::*;

    #[cfg_attr(not(feature = "enum-u8"), derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "enum-u8", derive(Serialize_repr, Deserialize_repr), repr(u8))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(TS, EnumString)]
    #[ts(export)]
    pub enum Rarity {
        Special,
        Common,
        Rare,
        Epic,
        Legendary,
        Ancient
    }

    #[derive(Serialize, Deserialize, TS)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct RarityChances {
        pub rarity: Option<Rarity>,
        pub cookie: f32,
        pub soulstone: f32
    }
}