use std::{io::{self, Write}, str::FromStr};

use regex::Regex;

use scraper::{Html, Selector};

mod t_characters;
use crate::t_characters::{Character, CharacterRarity, CharacterType, CharacterSelectors};

struct Scraper {
    base_url: String,
    client: reqwest::Client,
    selectors: CharacterSelectors
}
impl Scraper {
    fn new() -> Self {
        let inst = Self {
            base_url: "https://cookierunkingdom.fandom.com".to_string(),
            client: reqwest::Client::new(),
            selectors: CharacterSelectors::new()
        };
        return inst
    }
    async fn get_characters_urls(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut urls:Vec<String> = vec![];
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, "/wiki/List_of_Cookies")).send().await?.text().await?);
        let selector = Selector::parse(".wikitable > tbody th > a:not(.image)").unwrap();
        for element in document.select(&selector) {
            
            urls.push(element.value().attr("href").unwrap_or_default().to_string());
        }
        Ok(urls)
    }
    async fn get_character(&self, url: &String) -> Result<Character, Box<dyn std::error::Error>> {
        let document = Html::parse_document(&self.client.get(format!("{}{}", &self.base_url, url)).send().await?.text().await?);
        let mut temptype = document.select(&self.selectors.r#type).last().unwrap().text().collect::<String>();
        temptype.remove(0);
        let characterinst = Character {
            name: document.select(&self.selectors.name)
                .next().unwrap().inner_html().replace("\t", "").replace("\n", ""),
            r#type: CharacterType::from_str(temptype.as_str()).unwrap_or(CharacterType::Null),
            imagepath: Regex::new(r"/revision/.*").unwrap().replace(document.select(&self.selectors.imagepath)
            .next().unwrap().value().attr("src").unwrap(), "").to_string(),
            rarity: CharacterRarity::from_str(document.select(&self.selectors.rarity)
            .next().unwrap().value().attr("alt").unwrap().replace("\"", "").as_str()).unwrap_or(CharacterRarity::Null)
        };
        Ok(characterinst)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scraper = Scraper::new();
    let allcharactersurls = scraper.get_characters_urls().await?;
    println!("Getting Info for {} Cookies", allcharactersurls.len());
    let mut allcharacters:Vec<Character> = vec![];
    for (i, url) in allcharactersurls.iter().enumerate() {
        let character = scraper.get_character(url).await?;
        print!("\r\x1b[K{:.1}% Done | Cookie {} of {} | {}", (i as f32/allcharactersurls.len() as f32)*100.0, i+1, allcharactersurls.len(), character.name);
        io::stdout().flush().unwrap();
        allcharacters.push(character);
    }
    println!("{}", serde_json::to_string_pretty(&allcharacters).unwrap());
    Ok(())
}
