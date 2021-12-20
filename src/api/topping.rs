use std::{str::FromStr, vec};

use regex::Regex;

use scraper::{Html, Selector, ElementRef};

use treasure_types::*;
use crate::{api::rarity::{rarity_types::{Rarity}}, tools::ClientWrapper};

pub struct ToppingTools<'a> {
    pub clientwrapper: &'a ClientWrapper,
}
impl<'a> ToppingTools<'a> {
    pub fn from_clientwrapper(clientwrapper: &'a ClientWrapper) -> Self {
        let inst = Self {
            clientwrapper: clientwrapper
        };
        return inst
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
                    illustration_path: Regex::new(r"/revision/.*").unwrap().replace(first_child.attr("href").unwrap(), "").to_string(),
                    rarity: Rarity::from_str(raritytype.as_str()).unwrap()
                })
            }
        }
        Ok(temptreasures)
    }
    
}

pub mod topping_types {
    use crate::api::rarity::{rarity_types::{Rarity}};

    use serde::{Serialize, Deserialize};
    use ts_rs::TS;

    #[cfg(feature = "enum-u8")]
    use serde_repr::*;

    #[derive(Serialize, Deserialize, TS)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[ts(export)]
    #[serde(rename_all = "camelCase")]
    pub struct Treasure {
        pub name: String,
        pub illustration_path: String,
        pub rarity: Rarity
    }
}