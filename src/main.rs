use std::{fs, io::{self, Write}, ops::Add, path::Path};

mod scraper;
use crate::scraper::{Scraper, TScraper::Character};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut basepathtmp = "./scraper_output".to_owned();
    if !basepathtmp.ends_with(std::path::MAIN_SEPARATOR) { basepathtmp = basepathtmp.add(std::path::MAIN_SEPARATOR.to_string().as_str()); }
    let basepath = Path::new(&basepathtmp);
    let jsonpath = basepath.join("cookies.json");
    let assetspath = basepath.join("assets");

    fs::create_dir_all(&assetspath).expect("Could not access fs.");
    println!("Output Directory: {:?}", fs::canonicalize(&basepath).unwrap());
    
    let scraper = Scraper::new();
    let allcharactersurls = scraper.get_characters_urls().await?;
    println!("Getting Info for {} Cookies", allcharactersurls.len());
    let mut allcharacters:Vec<Character> = vec![];
    for (i, url) in allcharactersurls.iter().enumerate() {
        if i == 4 {break;}
        let mut character = scraper.get_character(url).await?;
        
        // Save image
        let imageres = scraper.client.get(&character.imagepath).send().await?.bytes().await?;
        let imagepath = &assetspath.join(character.name.to_owned() + ".png");
        fs::write(imagepath, imageres).expect("Image could not be written.");
        let temprelpath = imagepath.to_str().unwrap().to_owned().replace(&basepath.to_str().unwrap(), "./");
        character.imagepath = temprelpath;
        print!("\r\x1b[K{:.1}% Done | Cookie {} of {} | {}", (i as f32/allcharactersurls.len() as f32)*100.0, i+1, allcharactersurls.len(), &character.name);
        io::stdout().flush().unwrap();
        allcharacters.push(character);
    }
    fs::write(jsonpath, serde_json::to_string_pretty(&allcharacters).unwrap()).expect("JSON could not be written.");
    Ok(())
}
