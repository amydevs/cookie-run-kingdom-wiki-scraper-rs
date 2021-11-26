use std::{fs, env, io::{self, Write}, ops::Add, path::Path};

mod character;
use crate::character::{Scraper, TScraper::Character};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // arguments parsed
    let mut saveimgflag = false;
    let mut saveraritychanceflag = false;
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        match arg.as_str() {
            "--save-img" => saveimgflag = true,
            "--save-chances" => saveraritychanceflag = true,
            &_ => continue
        }
    }
    
    // init
    let mut basepathtmp = "./scraper_output".to_owned();
    if !basepathtmp.ends_with(std::path::MAIN_SEPARATOR) { basepathtmp = basepathtmp.add(std::path::MAIN_SEPARATOR.to_string().as_str()); }
    let basepath = Path::new(&basepathtmp);
    let cookiesjsonpath = basepath.join("cookies.json");
    let charrarityjsonpath = basepath.join("cookies_rarity.json");
    let assetspath = basepath.join("assets");

    fs::create_dir_all(&assetspath).expect("Could not access fs.");
    println!("Output Directory: {:?}", fs::canonicalize(&basepath).unwrap());
    
    let scraper = Scraper::new();

    // rarity percentages
    if saveraritychanceflag {
        println!("Getting Gacha Chance Percentages");
        let allraritychance = scraper.get_rarity_chances().await.unwrap_or(vec![]);
        fs::write(charrarityjsonpath, serde_json::to_string_pretty(&allraritychance).unwrap()).expect("JSON could not be written.");
    }

    // url's of all characters
    let allcharactersurls = scraper.get_characters_urls().await?;
    println!("Getting Info for {} Cookies", allcharactersurls.len());
    // filling vector with characters
    let mut allcharacters:Vec<Character> = vec![];
    for (i, url) in allcharactersurls.iter().enumerate() {
        if i == 4 {break;}
        let mut character = scraper.get_character(url).await?;
        
        // Save image
        if saveimgflag {
            let imageres = scraper.client.get(&character.image_path).send().await?.bytes().await?;
            let imagepath = &assetspath.join(character.name.to_owned() + ".png");
            fs::write(imagepath, imageres).expect("Image could not be written.");
            let temprelpath = imagepath.to_str().unwrap().to_owned().replace(&basepath.to_str().unwrap(), "./");
            character.image_path = temprelpath;
        }

        print!("\r\x1b[K{:.1}% Done | Cookie {} of {} | {}", (i as f32/allcharactersurls.len() as f32)*100.0, i+1, allcharactersurls.len(), &character.name);
        io::stdout().flush().unwrap();
        allcharacters.push(character);
    }
    fs::write(cookiesjsonpath, serde_json::to_string_pretty(&allcharacters).unwrap()).expect("JSON could not be written.");
    Ok(())
}
