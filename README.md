# cookie-run-kingdom-wiki-scraper-rs

## Run
```bash
# [Basic]
cargo run

# [Optional Compile-Time Features]
# use-enum-u8 - Automatically represents enumerators as u8 enums.
# [Optional Arguments]
# --save-imgs 
# --save-chances 
# -save-treasures
cargo run --features use-enum-u8 -- --save-imgs --save-chances -save-treasures
```

## Generate Typescript Types
```bash
# [Basic]
cargo test

# [Generate With U8 Enums]
cargo test --features use-enum-u8
```