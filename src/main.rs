use std::{fs, env, io::{self, Write}, ops::Add, path::Path};

mod api;
mod tools;

use crate::{tools::*, api::{character::*, rarity::*, treasure::*}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // arguments parsed
    let mut saveimgflag = false;
    let mut saveraritychanceflag = false;
    let mut savetreasuresflag = false;
    let args: Vec<String> = env::args().collect();
    for arg in &args {
        match arg.as_str() {
            "--save-imgs" => saveimgflag = true,
            "--save-chances" => saveraritychanceflag = true,
            "--save-treasures" => savetreasuresflag = true,
            &_ => continue
        }
    }
    
    // init
    let mut basepathtmp = "./scraper_output".to_owned();
    if !basepathtmp.ends_with(std::path::MAIN_SEPARATOR) { basepathtmp = basepathtmp.add(std::path::MAIN_SEPARATOR.to_string().as_str()); }
    let basepath = Path::new(&basepathtmp);
    let cookiesjsonpath = basepath.join("cookies.json");
    let charrarityjsonpath = basepath.join("cookies_rarity.json");
    let treasuresjsonpath = basepath.join("treasures.json");
    let assetspath = basepath.join("assets");

    fs::create_dir_all(&assetspath).expect("Could not access fs.");
    println!("Output Directory: {:?}", fs::canonicalize(&basepath).unwrap());

    // init clientwrapper
    let clientwrapper = ClientWrapper::new();

    // url's of all characters
    let charactertools = CharacterTools::from_clientwrapper(&clientwrapper);
    let allcharactersurls = charactertools.get_characters_urls().await?;
    println!("Getting Info for {} Cookies", allcharactersurls.len());
    // filling vector with characters
    let mut allcharacters:Vec<character_types::Character> = vec![];
    for (i, url) in allcharactersurls.iter().enumerate() {
        let character = charactertools.get_character(url).await?;
        
        #[cfg(feature = "debug")]
        if i == 4 {break;}

        // Save image
        if saveimgflag {
            let imagefoldpath = &assetspath.join(character.name.to_owned());
            fs::create_dir_all(&imagefoldpath).expect("Could not access fs.");
            fs::write(&imagefoldpath.join("illustration.png"), &clientwrapper.client.get(&character.illustration_img_path).send().await?.bytes().await?).expect("Image could not be written.");
            fs::write(&imagefoldpath.join("soulstone.png"), &clientwrapper.client.get(&character.soulstone_img_path).send().await?.bytes().await?).expect("Image could not be written.");
        }

        print!("\r\x1b[K{:.1}% Done | Cookie {} of {} | {}", (i as f32/allcharactersurls.len() as f32)*100.0, i+1, allcharactersurls.len(), &character.name);
        io::stdout().flush().unwrap();
        allcharacters.push(character);
    }
    println!("");
    fs::write(cookiesjsonpath, serde_json::to_string_pretty(&allcharacters).unwrap()).expect("JSON could not be written.");

    // rarity percentages
    let raritytools = RarityTools::from_clientwrapper(&clientwrapper);
    if saveraritychanceflag {
        println!("Getting Gacha Chance Percentages");
        let allraritychance = raritytools.get_rarity_chances().await.unwrap_or(vec![]);
        fs::write(charrarityjsonpath, serde_json::to_string_pretty(&allraritychance).unwrap()).expect("JSON could not be written.");
    }

    // treasures (kinda buggy atm)
    let treasuretools = TreasureTools::from_clientwrapper(&clientwrapper);
    if savetreasuresflag {
        println!("Getting Treasures");
        let alltreasures = treasuretools.get_treasures().await.unwrap_or(vec![]);
        fs::write(treasuresjsonpath, serde_json::to_string_pretty(&alltreasures).unwrap()).expect("JSON could not be written.");
    }

    Ok(())
}

#[cfg(all(test, feature = "enum-u8"))]
mod tests {
    // ts bindings regex: (?<=(\||=)\s*").*?(?=")

    use super::*;
    use std::path::Path;
    use regex::{Match, Regex};

    #[test]
    fn repr_bindings() {
        let bindingspath = Path::new("./bindings");
        let regexsearch = Regex::new(r#""\S*?""#).unwrap();
        if bindingspath.exists() {
            let paths = fs::read_dir(bindingspath).unwrap();

            for path in paths {
                let actualpath = path.unwrap().path();
                let data = fs::read_to_string(&actualpath).unwrap();
                let firstline = data.lines().next().unwrap();
                if firstline.starts_with("export type ") {
                    let mut tempdata = String::from(format!("export enum {} {{\n", firstline.split("export type ").last().unwrap().split(" =").next().unwrap()));
                    regexsearch.find_iter(data.as_str()).enumerate().for_each(|(i, m)| {
                        let mut tempchars = m.as_str().chars();
                        tempchars.next();
                        tempchars.next_back();
                        let mut templine = format!("  {}", tempchars.as_str());
                        if i == 0 {
                            templine.push_str(format!(" = {}", i).as_str());
                        }
                        templine.push_str(",\n");
                        tempdata.push_str(templine.as_str());
                    });
                    tempdata.push_str("}");
                    fs::write(actualpath, tempdata).unwrap();
                }
            }
        }
    }
}