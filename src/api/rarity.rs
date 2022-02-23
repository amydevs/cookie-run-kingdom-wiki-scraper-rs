use std::{str::FromStr, vec};

use crate::tools::*;
use scraper::{Html, Selector};

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

        let thsel = Selector::parse("th,td").unwrap();

        let document = Html::parse_document(&self.clientwrapper.client.get(format!("{}{}", &self.clientwrapper.base_url, "/wiki/Gacha")).send().await?.text().await?);
        let table = document.select(&Selector::parse(".wds-tab__content > .wikitable").unwrap()).next().unwrap();

        for (ri, row) in table.select(&Selector::parse("tr").unwrap()).enumerate() {
            for (i, ele) in row.select(&Selector::parse("th,td").unwrap()).enumerate() {
                println!("{}\n",ele.html());
                if i != 0 {
                    if i > rarities.len(){
                        rarities.push(RarityChances {
                            rarity: None,
                            cookie: 0.0,
                            soulstone: 0.0,
                        });
                    }
                    match ri {
                        0 => rarities[i-1].rarity = Rarity::from_str(&ele.text().next().unwrap()).ok(),
                        1 => rarities[i-1].soulstone = getf32fromstr(&ele.text().next().unwrap()),
                        2 => rarities[i-1].cookie = getf32fromstr(&ele.text().next().unwrap()),
                        _ => println!("out of bounds")
                    }
                }
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